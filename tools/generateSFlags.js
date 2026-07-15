if (process.argv.length != 4 && process.argv.length != 5) {
  console.error(
    "Usage: " +
      process.argv[0] +
      " " +
      process.argv[1] +
      " path/to/escrow/rippled path/to/contract/rippled [path/to/pipe/to]",
  )
  console.error(
    "Both rippled paths may be local dirs or GitHub URLs, e.g. https://github.com/XRPLF/rippled/tree/ripple/smart-escrow",
  )
  console.error(
    "Escrow-side flags are sourced from (and always trust) the escrow branch, so a rename there is picked up automatically. " +
      "The contract branch is only used for flags/masks that don't exist on the escrow branch at all.",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")
const { readSourceFile: read } = require("./rippledSource")

// Strip C-style block comments so commented-out/deprecated/reserved entries
// (e.g. `/* ASF_FLAG(asfTshCollect, 11) */`) are never mistaken for real ones.
function stripComments(text) {
  return text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\r\n]*/g, "")
}

// Join backslash-newline continuations (used throughout the XMACRO body) into
// a single line so regexes don't need to account for line breaks mid-token.
function joinContinuations(text) {
  return text.replace(/\\\r?\n/g, " ")
}

function isNumericLiteral(token) {
  return /^0x[0-9a-fA-F]+$/.test(token) || /^\d+$/.test(token)
}

// Translate a verbatim C++ boolean/bitwise expression referencing already
// -emitted names into the Rust equivalent. `~` (bitwise NOT) is the only
// operator that differs; `|`/`&` and identifiers/hex literals carry over as-is.
function translateExpr(expr) {
  return expr.trim().replace(/~/g, "!")
}

function parseLsfMap(ledgerFormatsFileStripped) {
  const map = new Map()
  for (const [, name, value] of ledgerFormatsFileStripped.matchAll(
    /LSF_FLAG\(\s*(ls[a-zA-Z0-9]+)\s*,\s*(0x[0-9a-fA-F]+)\s*\)/g,
  )) {
    map.set(name, value)
  }
  return map
}

// Merges two Map<name, value> maps: `base` (e.g. escrow) is authoritative;
// entries in `extra` (e.g. contract) not already in `base` are added. Entries
// present in both with a different value are reported as conflicts.
function mergeMaps(base, extra, describeConflict) {
  const merged = new Map(base)
  const added = []
  let conflict = false
  for (const [name, value] of extra) {
    if (!merged.has(name)) {
      merged.set(name, value)
      added.push(name)
    } else if (merged.get(name) !== value) {
      console.error(describeConflict(name, merged.get(name), value))
      conflict = true
    }
  }
  return { merged, added, conflict }
}

// Parses the `XMACRO(TRANSACTION, TF_FLAG, TF_FLAG2, MASK_ADJ)` table plus
// the trailing standalone flags/masks/asf values out of one source's raw
// TxFlags.h text. Returns everything keyed so the caller can merge two
// sources' results before emitting any Rust.
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

  // Map<txName, {order, flagDecls: [[name, resolvedValue]], maskFlagNames: [], maskAdj}>
  const txBlocks = new Map()

  for (let i = 0; i < txStarts.length; ++i) {
    const name = txStarts[i][1]
    const blockStart = txStarts[i].index
    const blockEnd =
      i + 1 < txStarts.length ? txStarts[i + 1].index : xmacroBody.length
    const block = xmacroBody.substring(blockStart, blockEnd)

    const flagDecls = []
    const maskFlagNames = []

    for (const [, flagName, rawValue] of block.matchAll(
      /TF_FLAG\(\s*([A-Za-z0-9]+)\s*,\s*([^,()]+?)\s*\)/g,
    )) {
      flagDecls.push([flagName, resolveValue(rawValue)])
      maskFlagNames.push(flagName)
    }
    for (const [, flagName] of block.matchAll(
      /TF_FLAG2\(\s*([A-Za-z0-9]+)\s*,\s*[^,()]+?\s*\)/g,
    )) {
      // TF_FLAG2 references a flag already defined elsewhere in this same
      // table (e.g. AMMWithdraw reusing AMMDeposit's tfLPToken) -- contributes
      // to this transaction's mask but is not redeclared.
      maskFlagNames.push(flagName)
    }

    const maskAdjMatch = block.match(/MASK_ADJ\(\s*([^)]*)\)/)
    const maskAdj = maskAdjMatch ? maskAdjMatch[1].trim() : "0"

    txBlocks.set(name, { order: i, flagDecls, maskFlagNames, maskAdj })
  }

  // A few standalone constants (e.g. tfSendAmount/tfContractParameterMask)
  // are declared between the XMACRO's closing paren and the "// clang-format
  // on" marker -- textually inside the last TRANSACTION() block's substring,
  // but not matched by TF_FLAG/MASK_ADJ. Recover them by scanning everything
  // after the last real `MASK_ADJ(...)` call in the macro body.
  const maskAdjOccurrences = [...xmacroBody.matchAll(/MASK_ADJ\([^)]*\)\)/g)]
  const xmacroExtra =
    maskAdjOccurrences.length > 0
      ? xmacroBody.substring(
          maskAdjOccurrences[maskAdjOccurrences.length - 1].index +
            maskAdjOccurrences[maskAdjOccurrences.length - 1][0].length,
        )
      : ""

  ////////////////////////////////////////////////////////////////////////
  //  Trailer: additional standalone flags/masks/combos declared directly as
  //  `inline constexpr FlagValue NAME = EXPR;` or `constexpr std::uint32_t
  //  NAME = EXPR;` outside the XMACRO table (Universal transaction flags,
  //  MPToken mutable flags, NFToken legacy masks, AMM sub-tx combos,
  //  Contract parameter flags, etc).
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
  const escrowSource = process.argv[2]
  const contractSource = process.argv[3]

  const [
    escrowTxFlagsRaw,
    escrowLedgerFormats,
    contractTxFlagsRaw,
    contractLedgerFormats,
  ] = await Promise.all([
    read(escrowSource, "include/xrpl/protocol/TxFlags.h"),
    read(escrowSource, "include/xrpl/protocol/LedgerFormats.h").then(
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
  const escrowLsfMap = parseLsfMap(escrowLedgerFormats)
  const contractLsfMap = parseLsfMap(contractLedgerFormats)
  const {
    merged: lsfMap,
    added: lsfAdded,
    conflict: lsfConflict,
  } = mergeMaps(
    escrowLsfMap,
    contractLsfMap,
    (name, escrowVal, contractVal) =>
      `Conflict for ledger flag ${name}: escrow branch=${escrowVal} contract branch=${contractVal}`,
  )
  console.log(
    `📝 Ledger flags (lsf*): ${escrowLsfMap.size} from escrow branch, +${lsfAdded.length} contract-only additions, ${lsfMap.size} total`,
  )

  const escrowParsed = parseTxFlagsSource(escrowTxFlagsRaw, lsfMap)
  const contractParsed = parseTxFlagsSource(contractTxFlagsRaw, lsfMap)

  let anyConflict = lsfConflict

  ////////////////////////////////////////////////////////////////////////
  //  Merge per-transaction TF_FLAG/mask blocks: escrow's blocks are
  //  authoritative; transaction types that only exist on the contract
  //  branch (e.g. "Contract") are appended after, in the contract branch's
  //  own order.
  ////////////////////////////////////////////////////////////////////////
  const txBlocks = new Map(escrowParsed.txBlocks)
  const addedTxNames = []
  for (const [name, block] of contractParsed.txBlocks) {
    if (!txBlocks.has(name)) {
      txBlocks.set(name, block)
      addedTxNames.push(name)
      continue
    }
    const escrowFlagNames = new Set(txBlocks.get(name).maskFlagNames)
    const contractFlagNames = new Set(block.maskFlagNames)
    const onlyInContract = [...contractFlagNames].filter(
      (n) => !escrowFlagNames.has(n),
    )
    const onlyInEscrow = [...escrowFlagNames].filter(
      (n) => !contractFlagNames.has(n),
    )
    if (onlyInContract.length > 0 || onlyInEscrow.length > 0) {
      console.error(
        `Conflict for transaction ${name}: escrow and contract branches declare different flag sets ` +
          `(escrow-only: ${onlyInEscrow.join(", ") || "none"}; contract-only: ${onlyInContract.join(", ") || "none"}). Resolve manually.`,
      )
      anyConflict = true
    }
  }
  console.log(
    `📝 Transactions: ${escrowParsed.txBlocks.size} from escrow branch, +${addedTxNames.length} contract-only additions${
      addedTxNames.length > 0 ? ` (${addedTxNames.join(", ")})` : ""
    }, ${txBlocks.size} total`,
  )

  ////////////////////////////////////////////////////////////////////////
  //  Merge trailer entries and asf entries the same way.
  ////////////////////////////////////////////////////////////////////////
  const {
    merged: trailerEntries,
    added: trailerAdded,
    conflict: trailerConflict,
  } = mergeMaps(
    escrowParsed.trailerEntries,
    contractParsed.trailerEntries,
    (name, escrowVal, contractVal) =>
      `Conflict for flag ${name}: escrow branch="${escrowVal}" contract branch="${contractVal}"`,
  )
  anyConflict = anyConflict || trailerConflict
  console.log(
    `📝 Trailer flags: ${escrowParsed.trailerEntries.size} from escrow branch, +${trailerAdded.length} contract-only additions${
      trailerAdded.length > 0 ? ` (${trailerAdded.join(", ")})` : ""
    }, ${trailerEntries.size} total`,
  )

  const {
    merged: asfEntries,
    added: asfAdded,
    conflict: asfConflict,
  } = mergeMaps(
    escrowParsed.asfEntries,
    contractParsed.asfEntries,
    (name, escrowVal, contractVal) =>
      `Conflict for AccountSet flag ${name}: escrow branch=${escrowVal} contract branch=${contractVal}`,
  )
  anyConflict = anyConflict || asfConflict
  console.log(
    `📝 AccountSet flags (asf*): ${escrowParsed.asfEntries.size} from escrow branch, +${asfAdded.length} contract-only additions, ${asfEntries.size} total`,
  )

  if (anyConflict) {
    console.error(
      "\n❌ One or more flags differ between the escrow and contract branches -- see above. Aborting without writing output.",
    )
    process.exit(1)
  }

  const unresolved = new Set([
    ...escrowParsed.unresolved,
    ...contractParsed.unresolved,
  ])

  ////////////////////////////////////////////////////////////////////////
  //  Emit
  ////////////////////////////////////////////////////////////////////////
  let output = ""
  function addLine(line) {
    output += line + "\n"
  }

  addLine(
    "// Auto-generated by tools/generateSFlags.js from rippled's include/xrpl/protocol/TxFlags.h",
  )
  addLine("// Do not hand-edit; re-run scripts/generate-sflags.sh instead.")
  addLine("")
  addLine("#![allow(non_upper_case_globals)]")
  addLine("")

  // Emits one trailer-sourced constant (a standalone `inline constexpr
  // FlagValue NAME = EXPR;` from TxFlags.h, resolved against lsfMap/already
  // -emitted names where the expression isn't a bare numeric literal).
  function emitTrailerEntry(name, rawExpr) {
    if (isNumericLiteral(rawExpr)) {
      addLine(`pub const ${name}: u32 = ${rawExpr};`)
    } else if (
      !rawExpr.includes("|") &&
      !rawExpr.includes("~") &&
      lsfMap.has(rawExpr)
    ) {
      // Bare alias of an lsf*/lsmf* ledger-object flag, e.g. `= lsmfMPTCanMutateCanLock;`
      addLine(`pub const ${name}: u32 = ${lsfMap.get(rawExpr)};`)
    } else {
      addLine(`pub const ${name}: u32 = ${translateExpr(rawExpr)};`)
    }
  }

  const emittedFlagNames = new Set()

  addLine("// Universal Transaction flags:")
  for (const name of [
    "tfFullyCanonicalSig",
    "tfInnerBatchTxn",
    "tfUniversal",
    "tfUniversalMask",
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

  // Escrow's transactions first (in their original document order), then
  // contract-only transactions in their own document order.
  const orderedTxNames = [...txBlocks.entries()]
    .sort((a, b) => a[1].order - b[1].order)
    .map(([name]) => name)
  const escrowTxNames = orderedTxNames.filter((n) => !addedTxNames.includes(n))
  const contractOnlyTxNames = orderedTxNames.filter((n) =>
    addedTxNames.includes(n),
  )

  for (const name of [...escrowTxNames, ...contractOnlyTxNames]) {
    const block = txBlocks.get(name)
    for (const [flagName, resolvedValue] of block.flagDecls) {
      if (!emittedFlagNames.has(flagName)) {
        emittedFlagNames.add(flagName)
        addLine(`pub const ${flagName}: u32 = ${resolvedValue};`)
      }
    }
    if (block.maskFlagNames.length > 0) {
      let expr = `!(tfUniversal | ${block.maskFlagNames.join(" | ")})`
      if (block.maskAdj !== "0") {
        expr += ` | ${block.maskAdj}`
      }
      addLine(`pub const tf${name}Mask: u32 = ${expr};`)
    }
    addLine("")
  }

  for (const [name, rawExpr] of trailerEntries) {
    if (emittedFlagNames.has(name)) continue // already covered above
    emittedFlagNames.add(name)
    emitTrailerEntry(name, rawExpr)
  }

  addLine("")
  addLine("// AccountSet SetFlag/ClearFlag values")
  for (const [name, value] of asfEntries) {
    addLine(`pub const ${name}: u32 = ${value};`)
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
