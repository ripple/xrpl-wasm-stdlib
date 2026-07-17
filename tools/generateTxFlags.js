if (process.argv.length != 4 && process.argv.length != 5) {
  console.error(
    "Usage: " +
      process.argv[0] +
      " " +
      process.argv[1] +
      " path/to/base/rippled path/to/contract/rippled [path/to/pipe/to]",
  )
  console.error(
    "Both rippled paths may be local dirs (e.g. /path/to/rippled) or GitHub URLs (e.g. https://github.com/XRPLF/rippled/tree/ripple/se/supported).",
  )
  console.error(
    "Both branches are read live, so upstream renames are picked up automatically. " +
      "The base branch is authoritative; the contract branch only contributes flags that don't exist on the base branch at all.",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")
const { readSourceFile: read } = require("./rippledSource")

// Strip C-style comments (both `/* */` blocks and `//` lines) so commented-out
// /deprecated/reserved entries (e.g. `/* ASF_FLAG(asfTshCollect, 11) */`) are
// never mistaken for real ones.
function stripComments(text) {
  return text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\r\n]*/g, "")
}

// Join backslash-newline continuations (used throughout the XMACRO body) into
// a single line so regexes don't need to account for line breaks mid-token.
function joinContinuations(text) {
  return text.replace(/\\\r?\n/g, " ")
}

// True if `token` is a hex (0x...) or decimal integer literal, as opposed to a
// symbolic name (e.g. an lsf* alias) that still needs resolving.
function isNumericLiteral(token) {
  return /^0x[0-9a-fA-F]+$/.test(token) || /^\d+$/.test(token)
}

// True if `name` is a validity mask (any constant whose name contains "Mask",
// e.g. tfPaymentMask, tfNFTokenMintMaskWithoutMutable). Masks encode which flag
// combinations rippled accepts for a transaction; contract code only checks
// individual flags, so masks are dropped rather than emitted.
function isMask(name) {
  return name.includes("Mask")
}

// Translate a verbatim C++ bitwise expression referencing already-emitted
// names into the Rust equivalent. `~` (bitwise NOT) is the only operator that
// differs; `|`/`&` and identifiers/hex literals carry over as-is.
function translateExpr(expr) {
  return expr.trim().replace(/~/g, "!")
}

// Parses LedgerFormats.h into a Map<lsfName, hexValue>. These ledger-object
// flags are looked up when a TF_FLAG aliases one instead of giving its own
// literal (e.g. TF_FLAG(tfMPTCanLock, lsfMPTCanLock)).
function parseLsfMap(ledgerFormatsFileStripped) {
  const map = new Map()
  for (const [, name, value] of ledgerFormatsFileStripped.matchAll(
    /LSF_FLAG\(\s*(ls[a-zA-Z0-9]+)\s*,\s*(0x[0-9a-fA-F]+)\s*\)/g,
  )) {
    map.set(name, value)
  }
  return map
}

// Merges two Map<name, value> maps: `base` (the base rippled branch) is
// authoritative and `extra` (contract) only adds entries not already in
// `base`. The contract branch only introduces new flags for new transaction
// types, so it never redefines a base entry -- there is no conflict to detect.
function mergeMaps(base, extra) {
  const merged = new Map(base)
  const added = []
  for (const [name, value] of extra) {
    if (!merged.has(name)) {
      merged.set(name, value)
      added.push(name)
    }
  }
  return { merged, added }
}

// Parses one source's raw TxFlags.h text into the pieces we emit: the
// individual transaction flags from the XMACRO(TRANSACTION, TF_FLAG, ...)
// table, the standalone trailer constants, and the ASF_FLAG (asf*) values.
// The XMACRO's TF_FLAG2/MASK_ADJ columns exist only to build per-transaction
// validity masks, which we don't emit, so they are ignored. Everything is
// keyed so the caller can merge two sources' results before emitting any Rust.
function parseTxFlagsSource(txFlagsFileRaw, lsfMap) {
  const unresolved = new Set()
  function resolveValue(token) {
    token = token.trim()
    if (isNumericLiteral(token)) return token
    if (lsfMap.has(token)) return lsfMap.get(token)
    unresolved.add(token)
    return token // leave symbolic; caller must define it or this won't compile
  }

  const xmacroStart = txFlagsFileRaw.indexOf(
    "#define XMACRO(TRANSACTION, TF_FLAG, TF_FLAG2, MASK_ADJ)",
  )
  const xmacroEnd = txFlagsFileRaw.indexOf("// clang-format on", xmacroStart)
  const xmacroBody = joinContinuations(
    stripComments(txFlagsFileRaw.substring(xmacroStart, xmacroEnd)),
  )

  // Split the macro body into one chunk per TRANSACTION(Name, ...) call.
  const txStarts = [
    ...xmacroBody.matchAll(/TRANSACTION\(\s*([A-Za-z0-9]+)\s*,/g),
  ]

  // Map<txName, {order, flagDecls: [[name, resolvedValue]]}>
  const txBlocks = new Map()

  for (let i = 0; i < txStarts.length; ++i) {
    const name = txStarts[i][1]
    const blockStart = txStarts[i].index
    const blockEnd =
      i + 1 < txStarts.length ? txStarts[i + 1].index : xmacroBody.length
    const block = xmacroBody.substring(blockStart, blockEnd)

    const flagDecls = []
    for (const [, flagName, rawValue] of block.matchAll(
      /TF_FLAG\(\s*([A-Za-z0-9]+)\s*,\s*([^,()]+?)\s*\)/g,
    )) {
      flagDecls.push([flagName, resolveValue(rawValue)])
    }

    txBlocks.set(name, { order: i, flagDecls })
  }

  // A few standalone constants (e.g. tfSendAmount) are declared between the
  // XMACRO's closing paren and the "// clang-format on" marker -- textually
  // inside the last TRANSACTION() block's substring, but not matched by
  // TF_FLAG. Recover them by scanning everything after the last real
  // `MASK_ADJ(...)` call in the macro body.
  const maskAdjOccurrences = [...xmacroBody.matchAll(/MASK_ADJ\([^)]*\)\)/g)]
  const xmacroExtra =
    maskAdjOccurrences.length > 0
      ? xmacroBody.substring(
          maskAdjOccurrences[maskAdjOccurrences.length - 1].index +
            maskAdjOccurrences[maskAdjOccurrences.length - 1][0].length,
        )
      : ""

  ////////////////////////////////////////////////////////////////////////
  //  Trailer: additional standalone constants declared directly as
  //  `inline constexpr FlagValue NAME = EXPR;` or `constexpr std::uint32_t
  //  NAME = EXPR;` outside the XMACRO table (Universal transaction flags,
  //  MPToken mutable flags, AMM sub-tx combos, Contract parameter flags,
  //  etc). Validity masks in this region (e.g. the NFToken legacy masks) are
  //  skipped here so downstream only ever sees emittable flags.
  ////////////////////////////////////////////////////////////////////////
  // Drop preprocessor directive lines (#define/#pragma/#undef) -- several of
  // them (e.g. `#define TO_VALUE(name, value) inline constexpr FlagValue name
  // = value;`) are macro *definitions* whose parameter placeholders would
  // otherwise be mistaken for real flag declarations by the regex below.
  const trailerText = (
    stripComments(txFlagsFileRaw.substring(0, xmacroStart)) +
    xmacroExtra +
    stripComments(txFlagsFileRaw.substring(xmacroEnd))
  )
    .split("\n")
    .filter((line) => !/^\s*#/.test(line))
    .join("\n")

  // Map<name, rawExprTrimmed>
  const trailerEntries = new Map()
  for (const [, name, rawExpr] of trailerText.matchAll(
    /(?:inline constexpr FlagValue|constexpr std::uint32_t)\s+([A-Za-z0-9]+)\s*=\s*([^;]+);/g,
  )) {
    if (isMask(name)) continue
    trailerEntries.set(name, rawExpr.trim())
  }

  ////////////////////////////////////////////////////////////////////////
  //  AccountSet SetFlag/ClearFlag values (asf*)
  ////////////////////////////////////////////////////////////////////////
  const asfEntries = new Map()
  for (const [, name, value] of stripComments(txFlagsFileRaw).matchAll(
    /ASF_FLAG\(\s*(asf[A-Za-z0-9]+)\s*,\s*(\d+)\s*\)/g,
  )) {
    asfEntries.set(name, value)
  }

  return { txBlocks, trailerEntries, asfEntries, unresolved }
}

async function main() {
  const baseSource = process.argv[2]
  const contractSource = process.argv[3]

  const [
    baseTxFlagsRaw,
    baseLedgerFormats,
    contractTxFlagsRaw,
    contractLedgerFormats,
  ] = await Promise.all([
    read(baseSource, "include/xrpl/protocol/TxFlags.h"),
    read(baseSource, "include/xrpl/protocol/LedgerFormats.h").then(
      stripComments,
    ),
    read(contractSource, "include/xrpl/protocol/TxFlags.h"),
    read(contractSource, "include/xrpl/protocol/LedgerFormats.h").then(
      stripComments,
    ),
  ])

  // Ledger-object flags (lsf*/lsmf*) that some TF_FLAG entries alias instead
  // of giving their own literal, e.g. TF_FLAG(tfMPTCanLock, lsfMPTCanLock).
  // Merged upfront since either source's TF_FLAG table may reference either
  // side's lsf names, and the values are stable ledger-level constants.
  const baseLsfMap = parseLsfMap(baseLedgerFormats)
  const contractLsfMap = parseLsfMap(contractLedgerFormats)
  const { merged: lsfMap, added: lsfAdded } = mergeMaps(
    baseLsfMap,
    contractLsfMap,
  )
  console.log(
    `📝 Ledger flags (lsf*, read only to resolve tx-flag aliases): ${baseLsfMap.size} from base branch, +${lsfAdded.length} contract-only additions, ${lsfMap.size} total`,
  )

  const baseParsed = parseTxFlagsSource(baseTxFlagsRaw, lsfMap)
  const contractParsed = parseTxFlagsSource(contractTxFlagsRaw, lsfMap)

  ////////////////////////////////////////////////////////////////////////
  //  Merge per-transaction TF_FLAG blocks: the base branch's blocks are
  //  authoritative; transaction types that only exist on the contract branch
  //  (e.g. "Contract") are appended after, in the contract branch's own order.
  //  The contract branch only adds new transaction types, so a shared type is
  //  never redefined -- base wins by simply keeping its block.
  ////////////////////////////////////////////////////////////////////////
  const txBlocks = new Map(baseParsed.txBlocks)
  const addedTxNames = []
  for (const [name, block] of contractParsed.txBlocks) {
    if (!txBlocks.has(name)) {
      txBlocks.set(name, block)
      addedTxNames.push(name)
    }
  }
  console.log(
    `📝 Transactions: ${baseParsed.txBlocks.size} from base branch, +${addedTxNames.length} contract-only additions${
      addedTxNames.length > 0 ? ` (${addedTxNames.join(", ")})` : ""
    }, ${txBlocks.size} total`,
  )

  ////////////////////////////////////////////////////////////////////////
  //  Merge trailer entries and asf entries the same way.
  ////////////////////////////////////////////////////////////////////////
  const { merged: trailerEntries, added: trailerAdded } = mergeMaps(
    baseParsed.trailerEntries,
    contractParsed.trailerEntries,
  )
  console.log(
    `📝 Trailer flags: ${baseParsed.trailerEntries.size} from base branch, +${trailerAdded.length} contract-only additions${
      trailerAdded.length > 0 ? ` (${trailerAdded.join(", ")})` : ""
    }, ${trailerEntries.size} total`,
  )

  const { merged: asfEntries, added: asfAdded } = mergeMaps(
    baseParsed.asfEntries,
    contractParsed.asfEntries,
  )
  console.log(
    `📝 AccountSet flags (asf*): ${baseParsed.asfEntries.size} from base branch, +${asfAdded.length} contract-only additions, ${asfEntries.size} total`,
  )

  const unresolved = new Set([
    ...baseParsed.unresolved,
    ...contractParsed.unresolved,
  ])

  ////////////////////////////////////////////////////////////////////////
  //  Emit. Validity masks were already dropped during parsing, so only
  //  individual flags reach this point.
  ////////////////////////////////////////////////////////////////////////
  let output = ""
  function addLine(line) {
    output += line + "\n"
  }

  addLine(
    "// Auto-generated by tools/generateTxFlags.js from rippled's include/xrpl/protocol/TxFlags.h",
  )
  addLine("// Do not hand-edit; re-run scripts/generate-tx-flags.sh instead.")
  addLine("")
  addLine("#![allow(non_upper_case_globals, dead_code)]")
  addLine("")

  // Emits one trailer-sourced constant (a standalone `inline constexpr
  // FlagValue NAME = EXPR;` from TxFlags.h, resolved against lsfMap/already
  // -emitted names where the expression isn't a bare numeric literal).
  function emitTrailerEntry(name, rawExpr) {
    if (isNumericLiteral(rawExpr)) {
      addLine(`pub(crate) const ${name}: u32 = ${rawExpr};`)
    } else if (
      !rawExpr.includes("|") &&
      !rawExpr.includes("~") &&
      lsfMap.has(rawExpr)
    ) {
      // Bare alias of an lsf*/lsmf* ledger-object flag, e.g. `= lsmfMPTCanMutateCanLock;`
      addLine(`pub(crate) const ${name}: u32 = ${lsfMap.get(rawExpr)};`)
    } else {
      addLine(`pub(crate) const ${name}: u32 = ${translateExpr(rawExpr)};`)
    }
  }

  const emittedFlagNames = new Set()

  addLine("// Universal Transaction flags:")
  for (const name of [
    "tfFullyCanonicalSig",
    "tfInnerBatchTxn",
    "tfUniversal",
  ]) {
    if (!trailerEntries.has(name)) {
      console.error(
        `Error: expected universal flag '${name}' not found in TxFlags.h -- ` +
          "rippled's preamble format may have changed; update parseTxFlagsSource.",
      )
      process.exit(1)
    }
    emitTrailerEntry(name, trailerEntries.get(name))
    emittedFlagNames.add(name)
  }
  addLine("")

  // Base-branch transactions first (in their original document order), then
  // contract-only transactions in their own document order.
  const orderedTxNames = [...txBlocks.entries()]
    .sort((a, b) => a[1].order - b[1].order)
    .map(([name]) => name)
  const baseTxNames = orderedTxNames.filter((n) => !addedTxNames.includes(n))
  const contractOnlyTxNames = orderedTxNames.filter((n) =>
    addedTxNames.includes(n),
  )

  for (const name of [...baseTxNames, ...contractOnlyTxNames]) {
    const block = txBlocks.get(name)
    let emittedAny = false
    for (const [flagName, resolvedValue] of block.flagDecls) {
      if (!emittedFlagNames.has(flagName)) {
        emittedFlagNames.add(flagName)
        addLine(`pub(crate) const ${flagName}: u32 = ${resolvedValue};`)
        emittedAny = true
      }
    }
    if (emittedAny) addLine("")
  }

  for (const [name, rawExpr] of trailerEntries) {
    if (emittedFlagNames.has(name)) continue // already covered above
    emittedFlagNames.add(name)
    emitTrailerEntry(name, rawExpr)
  }

  addLine("")
  addLine("// AccountSet SetFlag/ClearFlag values")
  for (const [name, value] of asfEntries) {
    addLine(`pub(crate) const ${name}: u32 = ${value};`)
  }

  if (unresolved.size > 0) {
    console.warn(
      "Warning: could not resolve these referenced values (left symbolic, will not compile as-is):",
      [...unresolved].join(", "),
    )
  }

  const outputFile =
    process.argv.length == 5
      ? process.argv[4]
      : path.join(__dirname, "../xrpl-wasm-stdlib/src/tx_flags.rs")
  try {
    await fs.writeFile(outputFile, output, "utf8")
    console.log("File written successfully to", outputFile)
  } catch (err) {
    console.error("Error writing to file:", err)
  }
}

main()
