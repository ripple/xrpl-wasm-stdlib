if (process.argv.length != 3 && process.argv.length != 4) {
  console.error(
    "Usage: " +
      process.argv[0] +
      " " +
      process.argv[1] +
      " path/to/rippled [outputDir]",
  )
  console.error(
    "rippled path may be a local dir or a GitHub URL, e.g. https://github.com/XRPLF/rippled/tree/ripple/smart-escrow",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")
const { readSourceFile: read } = require("./rippledSource")
const { resolveRustTypeDetailed } = require("./sfieldTypeMap")

// Strip C-style comments (both `/* */` blocks and `//` lines).
function stripComments(text) {
  return text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\r\n]*/g, "")
}

////////////////////////////////////////////////////////////////////////
//  ledger_entries.macro parsing
////////////////////////////////////////////////////////////////////////

// Returns a Map<sfFieldName (without "sf" prefix), {xrplType, ordinal}>, same
// shape produced by generateSFields.js's parseSfields.
function parseSfields(sfieldMacroFile) {
  const hits = [
    ...sfieldMacroFile.matchAll(
      /^ *[A-Z]*TYPED_SFIELD *\( *sf([^,\n]*),[ \n]*([^, \n]+)[ \n]*,[ \n]*([0-9]+)/gm,
    ),
  ]
  const map = new Map()
  for (const hit of hits) {
    map.set(hit[1], { xrplType: hit[2], ordinal: parseInt(hit[3]) })
  }
  return map
}

// Parses ledger_entries.macro into an ordered list of
// { entryName, classId, className, rpcName, fields: [{sfName, soe}] }.
// Handles both LEDGER_ENTRY(...) and LEDGER_ENTRY_DUPLICATE(...) -- the
// latter has the exact same argument shape and only exists to dodge a JSS
// name collision between a transaction and a ledger entry (e.g.
// DepositPreauth), so it's parsed identically.
function parseLedgerEntries(macroFile) {
  const stripped = stripComments(macroFile)
  const entries = []

  const entryRe =
    /LEDGER_ENTRY(?:_DUPLICATE)?\(\s*(lt[A-Z0-9_]+)\s*,\s*(0x[0-9a-fA-F]+)\s*,\s*([A-Za-z0-9]+)\s*,\s*([a-z0-9_]+)\s*,\s*\(\{([\s\S]*?)\}\s*\)\)/g

  for (const match of stripped.matchAll(entryRe)) {
    const [, entryName, classId, className, rpcName, fieldsBlock] = match
    const fields = []
    for (const [, sfName, soeRaw] of fieldsBlock.matchAll(
      /\{\s*sf([A-Za-z0-9]+)\s*,\s*(soe[A-Za-z]+)\s*\}/g,
    )) {
      fields.push({ sfName, soe: soeRaw.toUpperCase() })
    }
    entries.push({ entryName, classId, className, rpcName, fields })
  }

  return entries
}

////////////////////////////////////////////////////////////////////////
//  LedgerFormats.h parsing (per-entry lsf* flags)
////////////////////////////////////////////////////////////////////////

// Parses the `LEDGER_OBJECT(ClassName, LSF_FLAG(name, value) ...)` groups out
// of the XMACRO(LEDGER_OBJECT, LSF_FLAG, LSF_FLAG2) table in LedgerFormats.h.
// Returns a Map<className, [{name, value}]>. LSF_FLAG2 (used once, for
// MPToken's lsfMPTLocked alias) is treated the same as LSF_FLAG.
function parseLsfFlagsByClass(ledgerFormatsFileRaw) {
  // Locate the region on the raw text first -- the end marker is itself a
  // `//` comment, so it would disappear if we stripped comments beforehand.
  const xmacroStart = ledgerFormatsFileRaw.indexOf(
    "#define XMACRO(LEDGER_OBJECT, LSF_FLAG, LSF_FLAG2)",
  )
  const xmacroEnd = ledgerFormatsFileRaw.indexOf(
    "// clang-format on",
    xmacroStart,
  )
  const body = stripComments(
    ledgerFormatsFileRaw.substring(xmacroStart, xmacroEnd),
  )

  // Split into one chunk per `LEDGER_OBJECT(Name, ...)` group, the same way
  // generateTxFlags.js splits TxFlags.h's XMACRO body per `TRANSACTION(...)`.
  // A naive non-greedy `...\)\)` regex mis-terminates as soon as it sees ANY
  // "))" substring, which happens one LSF_FLAG early whenever a group has 2+
  // flags (the last flag's own closing paren plus LEDGER_OBJECT's closing
  // paren look identical to an inner flag's ")"  followed by the group's
  // final ")"). Slicing between successive `LEDGER_OBJECT(` starts sidesteps
  // needing to balance parens at all.
  const objectStarts = [
    ...body.matchAll(/LEDGER_OBJECT\(\s*([A-Za-z0-9]+)\s*,/g),
  ]
  const map = new Map()
  for (let i = 0; i < objectStarts.length; ++i) {
    const className = objectStarts[i][1]
    const chunkStart = objectStarts[i].index
    const chunkEnd =
      i + 1 < objectStarts.length ? objectStarts[i + 1].index : body.length
    const chunk = body.substring(chunkStart, chunkEnd)

    const flags = []
    for (const [, name, value] of chunk.matchAll(
      /LSF_FLAG2?\(\s*(ls[a-zA-Z0-9]+)\s*,\s*(0x[0-9a-fA-F]+)\s*\)/g,
    )) {
      flags.push({ name, value })
    }
    map.set(className, flags)
  }
  return map
}

////////////////////////////////////////////////////////////////////////
//  Doc-comment side map (tools/fieldDocs.json)
////////////////////////////////////////////////////////////////////////

async function loadFieldDocs() {
  const docsFile = path.join(__dirname, "fieldDocs.json")
  try {
    const text = await fs.readFile(docsFile, "utf8")
    return JSON.parse(text)
  } catch {
    return {}
  }
}

// Resolves a field's doc comment, preferring the most specific source:
//   1. an entry-qualified "<Class>.<Field>" key (e.g. "Escrow.Sequence"),
//   2. a bare "<Field>" key (generic fallback shared across entries),
//   3. a synthesized minimal doc.
// Entry-qualified keys exist because the same sfield name means different
// things on different objects (e.g. sfSequence, sfTransferRate on Escrow vs
// AccountRoot), so a bare-name lookup would otherwise splice AccountRoot prose
// onto an Escrow getter.
function docCommentFor(fieldDocs, className, sfName, soe) {
  const doc = fieldDocs[`${className}.${sfName}`] ?? fieldDocs[sfName]
  if (doc) {
    return doc
      .split("\n")
      .map((line) => (line.length > 0 ? `    /// ${line}` : "    ///"))
      .join("\n")
  }
  const requirement = soe === "SOEREQUIRED" ? "Required" : "Optional"
  return `    /// The ${sfName} field (${requirement}).`
}

////////////////////////////////////////////////////////////////////////
//  Field-name -> accessor-name conversion
////////////////////////////////////////////////////////////////////////

// Converts an sfield name (PascalCase, e.g. "NFTokenMinter") into the
// snake_case getter name used throughout traits.rs (e.g. "get_nf_token_minter").
// Mirrors the naming already used for hand-written getters: acronym runs
// (e.g. "NF", "ID", "AMM") are kept together rather than split letter-by-letter.
function toGetterName(sfName) {
  // Insert an underscore between a lowercase/digit and an uppercase letter,
  // and between an acronym run and a following Titlecase word (e.g.
  // "NFTokenMinter" -> "NF_Token_Minter", "AMMID" -> "AMMID" stays put but
  // "AMMID" as a whole field is handled as a single acronym below).
  let s = sfName
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
  return "get_" + s.toLowerCase()
}

////////////////////////////////////////////////////////////////////////
//  Output routing
////////////////////////////////////////////////////////////////////////

// Named output targets. Each target is a (crate, path) pair; `crate` drives
// whether stdlib items are imported via `crate::` (when the file lives IN
// xrpl-common-stdlib) or via `xrpl_common_stdlib::` (any other crate).
const TARGETS = {
  wasmStdlib: {
    crate: "xrpl-common-stdlib",
    path: "xrpl-common-stdlib/src/objects/",
  },
  escrowStdlib: {
    crate: "xrpl-escrow-stdlib",
    path: "xrpl-escrow-stdlib/src/ledger_objects/",
  },
}

// A generated entry produces three artifact kinds that may be routed
// independently:
//   - slotTrait:    `<Class>Fields` (slot-based, generic; generic consumers
//                   must be able to read the object by slot without pulling in
//                   any feature-specific crate).
//   - currentTrait: `Current<Class>Fields` (reads the current ledger object).
//   - struct:       `pub struct <Class>` + its `impl <Class>Fields`.
// Default for every entry: all three -> xrpl-common-stdlib. Per-entry overrides
// below relocate individual artifact kinds (e.g. escrow's current-object
// trait + concrete struct belong in the escrow crate, but its slot-based
// generic trait stays in wasm-stdlib).
const DEFAULT_ROUTING = {
  slotTrait: TARGETS.wasmStdlib,
  currentTrait: TARGETS.wasmStdlib,
  struct: TARGETS.wasmStdlib,
}

const ENTRY_ROUTING = {
  ltESCROW: {
    slotTrait: TARGETS.wasmStdlib,
    currentTrait: TARGETS.escrowStdlib,
    struct: TARGETS.escrowStdlib,
  },
}

function routingFor(entryName) {
  return ENTRY_ROUTING[entryName] || DEFAULT_ROUTING
}

// LedgerFormats.h's XMACRO(LEDGER_OBJECT, ...) table names a couple of
// objects differently than ledger_entries.macro's className column. Maps
// ledger_entries.macro's className -> the LEDGER_OBJECT name used in
// LedgerFormats.h, for the one known divergence (DirectoryNode/DirNode).
const LSF_CLASS_NAME_ALIASES = {
  DirectoryNode: "DirNode",
}

function lsfClassNameFor(className) {
  return LSF_CLASS_NAME_ALIASES[className] || className
}

////////////////////////////////////////////////////////////////////////
//  Common-field-set derivation
////////////////////////////////////////////////////////////////////////

// Fields that are synthetic / already covered elsewhere and must never be
// emitted as per-entry getters.
// - LedgerIndex: not a real field on the object -- it's a synthetic value
//   derived from the object's keylet/index. See the NOTE in traits.rs.
// - Flags / LedgerEntryType: covered by LedgerObjectCommonFields /
//   CurrentLedgerObjectCommonFields, so per-entry traits must not redeclare them.
const EXCLUDED_FIELDS = new Set(["LedgerIndex", "Flags", "LedgerEntryType"])

// Per-entry "<Entry>.<Field>" pairs whose getter is intentionally hand-written
// and must NOT be generated (in EITHER the slot or current trait). Unlike a
// field that fell through the type map, these have a bespoke Rust surface the
// generic getter cannot reproduce, so a `// SKIPPED` marker is emitted in
// place of the getter to make the omission explicit.
//   - Escrow.Data: hand-written `get_data`/`update_current_escrow_data` in the
//     escrow crate use ContractData + a bespoke buffer + the `update_data`
//     host call; the generic Blob getter would silently change its type.
const HANDWRITTEN_FIELDS = new Map([
  ["Escrow.Data", "hand-written (ContractData semantics)"],
])

////////////////////////////////////////////////////////////////////////
//  Rust emission
////////////////////////////////////////////////////////////////////////

const BANNER = [
  "// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.",
  "",
]

// Blob type aliases that live in crate::types::blob.
const BLOB_TYPES = [
  "StandardBlob",
  "ConditionBlob",
  "FulfillmentBlob",
  "SignatureBlob",
  "UriBlob",
  "WasmBlob",
  "PublicKeyBlob",
]

// Resolves each field of an entry to one of three dispositions:
//   { kind: "field",  field, rustType }         -> emit a getter
//   { kind: "skip",   field, reason }           -> emit a `// SKIPPED` marker
// EXCLUDED_FIELDS (Flags/LedgerEntryType/LedgerIndex) are dropped entirely
// (no marker) since they are covered by the common trait / are synthetic.
// A field is SKIPPED when either (a) it's on the hand-written list, or (b) its
// XRPL wire type has no genuine Rust mapping and only *would* fall back to u8
// -- emitting a u8 getter for a 32-byte VECTOR256 (etc.) is a runtime footgun.
function resolveEntryFields(entry, sfieldTypes, skipped) {
  const out = []
  for (const field of entry.fields) {
    if (EXCLUDED_FIELDS.has(field.sfName)) continue

    const handwrittenKey = `${entry.className}.${field.sfName}`
    if (HANDWRITTEN_FIELDS.has(handwrittenKey)) {
      const reason = HANDWRITTEN_FIELDS.get(handwrittenKey)
      out.push({ kind: "skip", field, reason })
      skipped.push({ entry: entry.className, sfName: field.sfName, reason })
      continue
    }

    const def = sfieldTypes.get(field.sfName)
    if (!def) {
      const reason = "not found in sfields.macro"
      out.push({ kind: "skip", field, reason })
      skipped.push({ entry: entry.className, sfName: field.sfName, reason })
      continue
    }

    const { rustType, source } = resolveRustTypeDetailed(
      field.sfName,
      def.xrplType,
    )
    if (source === "none") {
      // Wire type is real but not yet representable (VECTOR256, XCHAIN_BRIDGE,
      // NUMBER, INT32, JSON, ...). Do NOT fabricate a u8 getter.
      const reason = `${def.xrplType} is not yet representable in Rust`
      out.push({ kind: "skip", field, reason, wireType: def.xrplType })
      skipped.push({
        entry: entry.className,
        sfName: field.sfName,
        reason,
        wireType: def.xrplType,
      })
      continue
    }

    out.push({ kind: "field", field, rustType })
  }
  return out
}

// The `use` prefix for stdlib items depends on where the file lives: files IN
// xrpl-common-stdlib reference it as `crate::`; files in any other crate must
// use the external crate path `xrpl_common_stdlib::`.
function stdlibPrefix(crate) {
  return crate === "xrpl-common-stdlib" ? "crate" : "xrpl_common_stdlib"
}

// Builds the `use` block for a file, given the target crate and the set of
// Rust types actually referenced by artifacts written into that file. Only
// the modules a file actually needs are imported.
function buildImports(crate, usedRustTypes, opts) {
  const p = stdlibPrefix(crate)
  const lines = []

  // Common-trait imports depend on which traits the file references.
  const commonTraits = []
  if (opts.needsSlotCommon) commonTraits.push("LedgerObjectCommonFields")
  if (opts.needsCurrentCommon)
    commonTraits.push("CurrentLedgerObjectCommonFields")
  if (commonTraits.length > 0) {
    commonTraits.sort()
    lines.push(
      commonTraits.length === 1
        ? `use ${p}::objects::traits::${commonTraits[0]};`
        : `use ${p}::objects::traits::{${commonTraits.join(", ")}};`,
    )
  }

  // Accessor modules: ledger_object (slot) and/or current_ledger_object.
  const accessors = []
  if (opts.needsSlotAccessor) accessors.push("ledger_object")
  if (opts.needsCurrentAccessor) accessors.push("current_ledger_object")
  if (accessors.length > 0) {
    accessors.sort()
    lines.push(
      accessors.length === 1
        ? `use ${p}::objects::${accessors[0]};`
        : `use ${p}::objects::{${accessors.join(", ")}};`,
    )
  }

  function pushGrouped(modulePath, items) {
    if (items.length === 0) return
    const sorted = [...items].sort()
    lines.push(
      sorted.length === 1
        ? `use ${modulePath}::${sorted[0]};`
        : `use ${modulePath}::{${sorted.join(", ")}};`,
    )
  }

  if (usedRustTypes.has("AccountID"))
    lines.push(`use ${p}::types::account_id::AccountID;`)
  if (usedRustTypes.has("Amount"))
    lines.push(`use ${p}::types::amount::Amount;`)
  if (usedRustTypes.has("Currency"))
    lines.push(`use ${p}::types::currency::Currency;`)
  if (usedRustTypes.has("Issue")) lines.push(`use ${p}::types::issue::Issue;`)

  // Array/Object are placeholder types that implement `LedgerObjectFieldGetter` as
  // no-ops; they live in `ledger_objects::array_object`, NOT `core::types::{array,object}`
  // (which intentionally do not implement the getter trait). This mirrors sfield.rs's import.
  const arrayObjectTypes = [...usedRustTypes].filter(
    (t) => t === "Array" || t === "Object",
  )
  pushGrouped(`${p}::objects::array_object`, arrayObjectTypes)

  const blobTypes = [...usedRustTypes].filter((t) => BLOB_TYPES.includes(t))
  pushGrouped(`${p}::types::blob`, blobTypes)

  const hashTypes = [...usedRustTypes].filter((t) => t.startsWith("Hash"))
  pushGrouped(`${p}::types::uint`, hashTypes)

  if (opts.needsResult) lines.push(`use ${p}::host::Result;`)
  if (opts.needsSfield) lines.push(`use ${p}::sfield;`)

  return lines
}

// Renders a single getter method's lines for either the slot-based or the
// current-object trait.
function renderGetter(entry, field, rustType, fieldDocs, useCurrent) {
  const { sfName, soe } = field
  const getterName = toGetterName(sfName)
  const doc = docCommentFor(fieldDocs, entry.className, sfName, soe)
  const isOptional = soe === "SOEOPTIONAL" || soe === "SOEDEFAULT"
  const returnType = isOptional
    ? `Result<Option<${rustType}>>`
    : `Result<${rustType}>`
  const call = useCurrent
    ? `current_ledger_object::get_field${isOptional ? "_optional" : ""}(sfield::${sfName})`
    : `ledger_object::get_field${isOptional ? "_optional" : ""}(self.get_slot_num(), sfield::${sfName})`
  return [
    doc,
    `    fn ${getterName}(&self) -> ${returnType} {`,
    `        ${call}`,
    `    }`,
    "",
  ]
}

// Renders the body of one trait (slot-based or current), including SKIPPED
// markers, but NOT the surrounding imports. Returns { lines, usedRustTypes }.
function renderTrait(entry, resolved, fieldDocs, { useCurrent }) {
  const { className } = entry
  const usedRustTypes = new Set()
  const lines = []
  if (useCurrent) {
    lines.push(
      `/// Trait providing access to fields specific to the current ${className} object.`,
    )
    lines.push(
      `pub trait Current${className}Fields: CurrentLedgerObjectCommonFields {`,
    )
  } else {
    lines.push(
      `/// Trait providing access to fields specific to ${className} objects in any ledger.`,
    )
    lines.push(`pub trait ${className}Fields: LedgerObjectCommonFields {`)
  }
  for (const item of resolved) {
    if (item.kind === "skip") {
      lines.push(
        `    // SKIPPED ${toGetterName(item.field.sfName)}: ${item.reason}`,
      )
      lines.push("")
      continue
    }
    usedRustTypes.add(item.rustType)
    lines.push(
      ...renderGetter(entry, item.field, item.rustType, fieldDocs, useCurrent),
    )
  }
  if (lines[lines.length - 1] === "") lines.pop()
  lines.push("}")
  return { lines, usedRustTypes }
}

// Renders the `mod flags { ... }` block for an entry.
function renderFlagsModule(entry, lsfFlagsByClass) {
  const { className } = entry
  const lsfFlags = lsfFlagsByClass.get(lsfClassNameFor(className)) || []
  const lines = []
  lines.push(`/// \`lsf*\` flag constants for ${className} objects.`)
  lines.push(`#[allow(non_upper_case_globals, dead_code)]`)
  lines.push(`pub mod ${flagsModName(className)} {`)
  for (const { name, value } of lsfFlags) {
    lines.push(`    pub const ${name}: u32 = ${value};`)
  }
  if (lsfFlags.length === 0) {
    lines.push(
      `    // No lsf* flags are defined for ${className} in LedgerFormats.h.`,
    )
  }
  lines.push(`}`)
  return lines
}

// Since several entries may share one output file, a bare `mod flags` per
// entry would collide. Name each flags module after the entry (snake_case).
function flagsModName(className) {
  return (
    className
      .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
      .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
      .toLowerCase() + "_flags"
  )
}

// Renders the concrete `pub struct <Class>` + inherent `new` + common-fields
// impl + `impl <Class>Fields`.
function renderStruct(entry) {
  const { className } = entry
  return [
    `#[derive(Debug, Clone, Copy, Eq, PartialEq)]`,
    `pub struct ${className} {`,
    `    pub(crate) slot_num: i32,`,
    `}`,
    "",
    `impl ${className} {`,
    `    pub fn new(slot_num: i32) -> Self {`,
    `        Self { slot_num }`,
    `    }`,
    `}`,
    "",
    `impl LedgerObjectCommonFields for ${className} {`,
    `    fn get_slot_num(&self) -> i32 {`,
    `        self.slot_num`,
    `    }`,
    `}`,
    "",
    `impl ${className}Fields for ${className} {}`,
  ]
}

// Builds the per-artifact blocks for one entry. Each block records which
// output target it goes to, its rendered body lines, the Rust types it uses,
// and which imports it requires. The `flags` module rides along with the
// struct's target (it's most useful next to the concrete type).
function buildEntryArtifacts(
  entry,
  resolved,
  lsfFlagsByClass,
  fieldDocs,
  routing,
) {
  const artifacts = []

  const slot = renderTrait(entry, resolved, fieldDocs, { useCurrent: false })
  const hasGetter = resolved.some((r) => r.kind === "field")
  artifacts.push({
    target: routing.slotTrait,
    lines: slot.lines,
    usedRustTypes: slot.usedRustTypes,
    needs: {
      needsSlotCommon: true,
      needsSlotAccessor: hasGetter,
      needsResult: hasGetter,
      needsSfield: hasGetter,
    },
  })

  const current = renderTrait(entry, resolved, fieldDocs, { useCurrent: true })
  artifacts.push({
    target: routing.currentTrait,
    lines: current.lines,
    usedRustTypes: current.usedRustTypes,
    needs: {
      needsCurrentCommon: true,
      needsCurrentAccessor: hasGetter,
      needsResult: hasGetter,
      needsSfield: hasGetter,
    },
  })

  artifacts.push({
    target: routing.struct,
    lines: [
      ...renderFlagsModule(entry, lsfFlagsByClass),
      "",
      ...renderStruct(entry),
    ],
    usedRustTypes: new Set(),
    // The struct references LedgerObjectCommonFields (for get_slot_num) and
    // the entry's own <Class>Fields trait. The latter lives in the slotTrait
    // target -- when that's a DIFFERENT crate/file, the struct must import it.
    needs: {
      needsSlotCommon: true,
    },
    // Extra import the struct needs to name `<Class>Fields` when the trait is
    // defined elsewhere. Resolved during file assembly (needs cross-file info).
    structForEntry: entry,
    slotTraitTarget: routing.slotTrait,
  })

  return artifacts
}

async function main() {
  const rippledSource = process.argv[2]
  const outputDir = process.argv[3] || path.join(__dirname, "..")

  const [ledgerEntriesMacro, sfieldsMacro, ledgerFormatsRaw] =
    await Promise.all([
      read(rippledSource, "include/xrpl/protocol/detail/ledger_entries.macro"),
      read(rippledSource, "include/xrpl/protocol/detail/sfields.macro"),
      read(rippledSource, "include/xrpl/protocol/LedgerFormats.h"),
    ])

  const entries = parseLedgerEntries(ledgerEntriesMacro)
  const sfieldTypes = parseSfields(sfieldsMacro)
  const lsfFlagsByClass = parseLsfFlagsByClass(ledgerFormatsRaw)
  const fieldDocs = await loadFieldDocs()

  console.log(`Ledger entries parsed: ${entries.length}`)
  let totalGetters = 0
  const skipped = []

  // Build every artifact for every entry, tagged with its output target.
  const artifacts = []
  for (const entry of entries) {
    const routing = routingFor(entry.entryName)
    const resolved = resolveEntryFields(entry, sfieldTypes, skipped)
    totalGetters += resolved.filter((r) => r.kind === "field").length
    for (const artifact of buildEntryArtifacts(
      entry,
      resolved,
      lsfFlagsByClass,
      fieldDocs,
      routing,
    )) {
      const file = path.join(outputDir, artifact.target.path, "generated.rs")
      artifacts.push({ ...artifact, file })
    }
  }

  console.log(`Total getters emitted (across all entries): ${totalGetters}`)
  console.log(`Total fields skipped: ${skipped.length}`)
  if (skipped.length > 0) {
    console.log("Skipped fields:")
    for (const s of skipped) {
      console.log(`  ${s.entry}.sf${s.sfName}: ${s.reason}`)
    }
  }

  // Group artifacts by their output file.
  const byFile = new Map()
  for (const a of artifacts) {
    if (!byFile.has(a.file)) byFile.set(a.file, [])
    byFile.get(a.file).push(a)
  }

  for (const [file, group] of byFile) {
    const crate = group[0].target.crate

    // Union of what the file's artifacts need for imports.
    const usedRustTypes = new Set()
    const needs = {
      needsSlotCommon: false,
      needsCurrentCommon: false,
      needsSlotAccessor: false,
      needsCurrentAccessor: false,
      needsResult: false,
      needsSfield: false,
    }
    // Cross-file trait imports the structs in this file require (e.g. an
    // Escrow struct here needs `EscrowFields` defined in another file/crate).
    const externalTraitImports = new Set()

    for (const a of group) {
      for (const t of a.usedRustTypes) usedRustTypes.add(t)
      for (const k of Object.keys(needs)) {
        if (a.needs[k]) needs[k] = true
      }
      // A struct artifact whose <Class>Fields trait lives in a different file
      // must import that trait. When the slot trait is in the SAME file, the
      // trait is already in scope and no import is needed.
      if (a.structForEntry) {
        const traitFile = path.join(
          outputDir,
          a.slotTraitTarget.path,
          "generated.rs",
        )
        if (traitFile !== file) {
          const p = stdlibPrefix(crate)
          // Once wired in, the generated slot traits are expected to live at
          // the same path the hand-written ones do today
          // (<crate>::objects::traits). The `mod` wiring is a
          // later step; this import assumes that destination.
          externalTraitImports.add(
            `use ${p}::objects::traits::${a.structForEntry.className}Fields;`,
          )
        }
      }
    }

    const importLines = buildImports(crate, usedRustTypes, needs)
    for (const imp of externalTraitImports) importLines.push(imp)

    const lines = [...BANNER]
    if (importLines.length > 0) {
      lines.push(...importLines)
      lines.push("")
    }
    for (let i = 0; i < group.length; i++) {
      lines.push(...group[i].lines)
      lines.push("")
    }

    const fullContents = lines.join("\n")
    await fs.mkdir(path.dirname(file), { recursive: true })
    await fs.writeFile(file, fullContents, "utf8")
    console.log(`Wrote ${group.length} artifact(s) to ${file}`)
  }

  console.log("")
  console.log(
    "Note: generated files were formatted with a standalone `rustfmt --edition 2024` pass. " +
      "They are consumed via `pub mod generated;` in each crate's ledger_objects/mod.rs " +
      "(xrpl-common-stdlib/src/objects/mod.rs and " +
      "xrpl-escrow-stdlib/src/ledger_objects/mod.rs). To add/edit doc comments, edit " +
      "tools/fieldDocs.json (keyed by sfield name).",
  )
}

main()
