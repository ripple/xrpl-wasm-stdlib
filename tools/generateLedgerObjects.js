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
const { execFileSync } = require("child_process")
const { readSourceFile: read } = require("./rippledSource")
const {
  resolveRustTypeDetailed,
  stripComments,
  parseSfields,
  BLOB_TYPES,
} = require("./sfieldTypeMap")

const XRPL_DEV_PORTAL_RAW =
  "https://raw.githubusercontent.com/XRPLF/xrpl-dev-portal/master/docs/references/protocol/ledger-data/ledger-entry-types"

////////////////////////////////////////////////////////////////////////
//  ledger_entries.macro parsing
////////////////////////////////////////////////////////////////////////

// Parses ledger_entries.macro into an ordered list of
// { entryName, classId, className, rpcName, fields: [{sfName, soe}] }.
// Handles both LEDGER_ENTRY(...) and LEDGER_ENTRY_DUPLICATE(...) -- the
// latter has the exact same argument shape and only exists to dodge a JSS
// name collision between a transaction and a ledger entry (e.g.
// DepositPreauth), so it's parsed identically.
function parseLedgerEntries(macroFile) {
  const stripped = stripComments(macroFile)
  const entries = []

  // Matches one entry, capturing entryName/classId/className/rpcName/fieldsBlock. e.g.:
  //   LEDGER_ENTRY(ltESCROW, 0x0075, Escrow, escrow, ({
  //     {sfAccount, soeREQUIRED},
  //     {sfDestination, soeREQUIRED},
  //   }))
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
//  xrpl-dev-portal doc-comment fetching
////////////////////////////////////////////////////////////////////////

// Cleans a raw markdown table-cell description into readable doc-comment
// prose: strips `{% ... %}` templating tags, converts `[text](url)` links to
// their visible text, and collapses/trims whitespace.
function cleanDescription(text) {
  return text
    .replace(/\{%[\s\S]*?%\}/g, "")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/\s+/g, " ")
    .trim()
}

// Parses the first "## ... Fields" table in an xrpl-dev-portal ledger-entry
// markdown file into a Map<fieldName, cleanedDescription>. Only the table
// immediately following the top-level "Fields" heading is used -- later
// "### <SubObject> Fields" tables describe nested/array sub-objects, not the
// entry's own top-level fields.
function parseFieldsTable(markdown) {
  const map = new Map()
  const headingMatch = markdown.match(/^##\s+.*Fields\s*$/m)
  if (!headingMatch) return map

  const start = headingMatch.index + headingMatch[0].length
  const nextHeadingMatch = markdown.slice(start).match(/^#{2,3}\s+/m)
  const section = nextHeadingMatch
    ? markdown.slice(start, start + nextHeadingMatch.index)
    : markdown.slice(start)

  // Matches one markdown table row, capturing the field name (col 1) and its
  // description (last col). e.g. for the row:
  //   | `Account` | String | AccountID | Yes | The owner of the escrow. |
  // captures field="Account", description=" The owner of the escrow. ".
  const rowRe =
    /^\|\s*`?([A-Za-z0-9]+)`?[^|]*\|[^|]*\|[^|]*\|[^|]*\|(.*)\|\s*$/gm
  for (const [, field, description] of section.matchAll(rowRe)) {
    map.set(field, cleanDescription(description))
  }
  return map
}

// Fetches the field-description map for one entry's xrpl-dev-portal page.
// Returns an empty map if the page or its Fields table can't be found --
// callers fall back to a generic doc comment in that case. Called once per
// entry (each entry is a distinct page), so no caching is needed.
async function fetchFieldDocs(className) {
  const key = className.toLowerCase()
  let markdown
  try {
    const response = await fetch(`${XRPL_DEV_PORTAL_RAW}/${key}.md`)
    markdown = response.ok ? await response.text() : ""
  } catch {
    markdown = ""
  }
  return parseFieldsTable(markdown)
}

// Resolves a field's doc comment: prefers the cleaned xrpl-dev-portal
// description, falling back to a generic minimal doc when the page or field
// row isn't found.
function docCommentFor(fieldDocsForClass, sfName, soe) {
  const description = fieldDocsForClass.get(sfName)
  if (description) {
    return `    /// ${description}`
  }
  const requirement = soe === "SOEREQUIRED" ? "Required" : "Optional"
  return `    /// The ${sfName} field (${requirement}).`
}

////////////////////////////////////////////////////////////////////////
//  Field-name -> accessor-name conversion
////////////////////////////////////////////////////////////////////////

const ABBREVIATIONS = [
  "AMM",
  "DID",
  "ID",
  "LP",
  "MPToken",
  "NFT",
  "NFToken",
  "NFTs",
  "NoRipple",
  "UNL",
  "URI",
  "XChain",
]

const CAMEL_TO_SNAKE_CASE_REGEX = /^[^A-Z]+|[A-Z]+(?![^A-Z])|[A-Z][^A-Z]*/g

function pyCapitalize(s) {
  return s.charAt(0).toUpperCase() + s.slice(1).toLowerCase()
}

// Converts an sfield name (PascalCase, e.g. "NFTokenID", "AMMID") into the
// snake_case accessor name used throughout the generated traits (e.g.
// "nftoken_id", "amm_id"). No `get_` prefix -- per the Rust API Guidelines
// (C-GETTER), idiomatic getters are named for the field, not `get_<field>`.
function toGetterName(sfName) {
  let field = sfName
  for (const abbreviation of ABBREVIATIONS) {
    if (field.includes(abbreviation)) {
      field = field.split(abbreviation).join(pyCapitalize(abbreviation))
    }
  }
  const words = [...field.matchAll(CAMEL_TO_SNAKE_CASE_REGEX)].map((m) =>
    m[0].toLowerCase(),
  )
  return words.join("_")
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

// Entries generated as "slot-only" (keyed by className): the slot-based
// `<Class>Fields` trait and the `<Class>` struct are emitted, but the
// current-object `Current<Class>Fields` trait is NOT. Escrow's current-object
// accessors are hand-written in xrpl-escrow-stdlib because they carry bespoke,
// host-mutable ContractData (`Data`) semantics plus an `update_data` mutation
// the generator doesn't model. The slot-based half is still generated so
// contracts can read *other* (non-current) escrows through a typed trait.
const SLOT_ONLY_ENTRIES = new Set(["Escrow"])

// Per-entry field exclusions, keyed by className. Escrow's `Data` field is a
// host-mutable ContractData blob whose accessor is hand-written in
// xrpl-escrow-stdlib (the `EscrowContractData` extension trait), so it must not
// be emitted here as a generic StandardBlob getter.
const PER_ENTRY_EXCLUDED_FIELDS = {
  Escrow: new Set(["Data"]),
}

// Rust types with a single, fixed import path, used for field-getter return
// types that don't live in crate::types::blob or crate::types::uint.
const SIMPLE_TYPE_IMPORTS = {
  AccountID: "crate::types::account_id::AccountID",
  Amount: "crate::types::amount::Amount",
  Currency: "crate::types::currency::Currency",
  Issue: "crate::types::issue::Issue",
  MptId: "crate::types::mpt_id::MptId",
}

////////////////////////////////////////////////////////////////////////
//  Rust emission
////////////////////////////////////////////////////////////////////////

const BANNER =
  "// GENERATED -- do not hand-edit. Run scripts/generate-ledger-objects.sh to regenerate.\n"

// Fixed-size placeholder capacity for fields whose XRPL wire type has no
// genuine Rust mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...).
// These getters return raw, unparsed bytes rather than a semantic type.
const RAW_UNMAPPED_FIELD_SIZE_CONST = "RAW_UNMAPPED_FIELD_SIZE"
const RAW_UNMAPPED_FIELD_SIZE = 512

// Resolves each field of an entry to one of two dispositions:
//   { kind: "field", field, rustType }  -> emit a getter for a mapped type
//   { kind: "raw",   field, wireType }  -> emit a raw `[u8; N]` placeholder
//                                          getter for an unmapped wire type
// EXCLUDED_FIELDS (Flags/LedgerEntryType/LedgerIndex) are dropped entirely.
function resolveEntryFields(entry, sfieldTypes, unmapped) {
  const perEntryExcluded = PER_ENTRY_EXCLUDED_FIELDS[entry.className]
  const out = []
  for (const field of entry.fields) {
    if (EXCLUDED_FIELDS.has(field.sfName)) continue
    if (perEntryExcluded && perEntryExcluded.has(field.sfName)) continue

    const def = sfieldTypes.get(field.sfName)
    if (!def) {
      // Not found in sfields.macro -- shouldn't normally happen since
      // ledger_entries.macro and sfields.macro come from the same rippled
      // source, but skip defensively rather than crash.
      continue
    }

    const { rustType, source } = resolveRustTypeDetailed(
      field.sfName,
      def.xrplType,
    )
    if (source === "none") {
      out.push({ kind: "raw", field, wireType: def.xrplType })
      unmapped.push({
        entry: entry.className,
        sfName: field.sfName,
        wireType: def.xrplType,
      })
      continue
    }

    // ARRAY/OBJECT fields can't be decoded into a value whole (they don't implement
    // `FromLedger`); they're reached element-by-element via the `path()` nested-field
    // builder on the common base trait. Emit no flat getter for them.
    if (rustType === "Array" || rustType === "Object") {
      continue
    }

    out.push({ kind: "field", field, rustType })
  }
  return out
}

// Renders a single getter method's lines for either the slot-based or the
// current-object trait.
function renderGetter(field, rustType, fieldDocsForClass, useCurrent) {
  const { sfName, soe } = field
  const getterName = toGetterName(sfName)
  const doc = docCommentFor(fieldDocsForClass, sfName, soe)
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

// Renders a raw placeholder getter for a field whose wire type has no
// genuine Rust mapping yet. Reads the field's raw bytes into a fixed-size
// buffer via the buffered host accessor and returns them as-is.
function renderRawGetter(field, wireType, fieldDocsForClass, useCurrent) {
  const { sfName, soe } = field
  const getterName = toGetterName(sfName)
  const doc = docCommentFor(fieldDocsForClass, sfName, soe)
  const isOptional = soe === "SOEOPTIONAL" || soe === "SOEDEFAULT"
  const returnType = isOptional
    ? `Result<Option<[u8; ${RAW_UNMAPPED_FIELD_SIZE_CONST}]>>`
    : `Result<[u8; ${RAW_UNMAPPED_FIELD_SIZE_CONST}]>`
  const hostCall = useCurrent
    ? `get_current_ledger_obj_field(sfield::${sfName}.into(), buffer.as_mut_ptr(), buffer.len())`
    : `get_ledger_obj_field(self.get_slot_num(), sfield::${sfName}.into(), buffer.as_mut_ptr(), buffer.len())`
  const matcher = isOptional
    ? `match_result_code_optional(result_code, || (result_code > 0).then_some(buffer))`
    : `match_result_code(result_code, || buffer)`
  return [
    doc,
    `    /// Raw bytes; ${wireType} is not yet typed in Rust.`,
    `    fn ${getterName}(&self) -> ${returnType} {`,
    `        let mut buffer = [0u8; ${RAW_UNMAPPED_FIELD_SIZE_CONST}];`,
    `        let result_code = unsafe { ${hostCall} };`,
    `        ${matcher}`,
    `    }`,
    "",
  ]
}

// Renders the body of one trait (slot-based or current), NOT the
// surrounding imports. Returns { lines, usedRustTypes, hasRawRequired,
// hasRawOptional }.
function renderTrait(entry, resolved, fieldDocsForClass, { useCurrent }) {
  const { className } = entry
  const usedRustTypes = new Set()
  let hasRawRequired = false
  let hasRawOptional = false
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
    if (item.kind === "raw") {
      const isOptional =
        item.field.soe === "SOEOPTIONAL" || item.field.soe === "SOEDEFAULT"
      if (isOptional) hasRawOptional = true
      else hasRawRequired = true
      lines.push(
        ...renderRawGetter(
          item.field,
          item.wireType,
          fieldDocsForClass,
          useCurrent,
        ),
      )
      continue
    }
    usedRustTypes.add(item.rustType)
    lines.push(
      ...renderGetter(item.field, item.rustType, fieldDocsForClass, useCurrent),
    )
  }
  if (lines[lines.length - 1] === "") lines.pop()
  lines.push("}")
  return { lines, usedRustTypes, hasRawRequired, hasRawOptional }
}

// Renders the concrete `pub struct <Class>` + inherent `new` + common-fields
// impl + `impl <Class>Fields`.
//
// A keylet-based `load()` constructor is intentionally NOT generated: keylet
// inputs vary per entry and the routing would be a large hand-maintained table
// that silently goes stale as entries are added. Callers construct a slot with
// `new(slot_num)` after caching a keylet from `crate::keylets` themselves; a
// per-entry `load()` can be hand-added where the ergonomics are worth it.
function renderStruct(entry) {
  const { className } = entry
  const lines = [
    `#[derive(Debug, Clone, Copy, Eq, PartialEq)]`,
    `pub struct ${className} {`,
    `    pub(crate) slot_num: i32,`,
    `}`,
    "",
    `impl ${className} {`,
    `    /// Binds this handle to a host-managed slot holding a ${className} ledger object.`,
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
  return { lines }
}

// Builds the `use` block for one generated file.
function buildImports(usedRustTypes, opts) {
  const lines = []

  const commonTraits = []
  if (opts.needsSlotCommon)
    commonTraits.push("crate::objects::traits::LedgerObjectCommonFields")
  if (opts.needsCurrentCommon)
    commonTraits.push("crate::objects::traits::CurrentLedgerObjectCommonFields")
  for (const t of commonTraits.sort()) lines.push(`use ${t};`)

  const accessors = []
  if (opts.needsSlotAccessor) accessors.push("ledger_object")
  if (opts.needsCurrentAccessor) accessors.push("current_ledger_object")
  if (accessors.length > 0) {
    accessors.sort()
    lines.push(
      accessors.length === 1
        ? `use crate::objects::${accessors[0]};`
        : `use crate::objects::{${accessors.join(", ")}};`,
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

  for (const [type, modulePath] of Object.entries(SIMPLE_TYPE_IMPORTS)) {
    if (usedRustTypes.has(type)) lines.push(`use ${modulePath};`)
  }

  // Array/Object are placeholder types that implement `LedgerObjectFieldGetter` as
  // no-ops; they live in `objects::array_object`, NOT `types::{array,object}`.
  const arrayObjectTypes = [...usedRustTypes].filter(
    (t) => t === "Array" || t === "Object",
  )
  pushGrouped("crate::objects::array_object", arrayObjectTypes)

  const blobTypes = [...usedRustTypes].filter((t) => BLOB_TYPES.includes(t))
  pushGrouped("crate::types::blob", blobTypes)

  const hashTypes = [...usedRustTypes].filter((t) => t.startsWith("Hash"))
  pushGrouped("crate::types::uint", hashTypes)

  if (opts.needsResult) lines.push(`use crate::host::Result;`)
  const matchFns = []
  if (opts.hasRawRequired) matchFns.push("match_result_code")
  if (opts.hasRawOptional) matchFns.push("match_result_code_optional")
  pushGrouped("crate::host::error_codes", matchFns)
  if (opts.hasRawGetter && opts.needsCurrentAccessor)
    lines.push(`use crate::host::get_current_ledger_obj_field;`)
  if (opts.hasRawGetter && opts.needsSlotAccessor)
    lines.push(`use crate::host::get_ledger_obj_field;`)
  if (opts.needsSfield) lines.push(`use crate::sfield;`)

  return lines
}

// Rust types with a fixed, known byte size whose `LedgerObjectFieldGetter` impl uses
// `match_result_code_with_expected_bytes[_optional]` (exact-size match). Used to decide
// whether a generated `optional_fields_none` test can additionally assert `.is_none()`
// for a given optional getter, on top of the blanket `.is_ok()` assertion that applies to
// every optional getter regardless of size class. Every other type (Amount, Issue, any
// Blob alias, Array/Object placeholders, and the raw `[u8; N]` unmapped-field
// placeholder) is variable-size or raw and only gets the blanket `.is_ok()` check, since
// FIELD_NOT_FOUND doesn't reliably map to `None` for those getters (see the module doc in
// generated/mod.rs and the semantics captured in match_result_code_optional).
const FIXED_SIZE_OPTIONAL_TYPES = new Set([
  "u8",
  "u16",
  "u32",
  "u64",
  "AccountID",
  "Currency",
  "Hash128",
  "Hash160",
  "Hash192",
  "Hash256",
])

// Renders the `#[cfg(test)] mod tests { ... }` block appended to a generated entry file.
// Kept thin: almost all mock-setup logic lives in `test_support.rs`; this only calls into
// it and asserts on the results.
function renderTestsBlock(entry, resolved) {
  const { className } = entry
  const requiredGetters = []
  const optionalGetters = []
  for (const item of resolved) {
    // Array/Object are placeholder types (used only for Location-based nested-field
    // navigation, e.g. iterating a PriceDataSeries or SignerEntries array element by
    // element) whose LedgerObjectFieldGetter impl unconditionally panics via
    // `unreachable!()` -- they must never be called directly, so the generated smoke
    // tests skip these getters entirely rather than exercising the panic path.
    if (
      item.kind === "field" &&
      (item.rustType === "Array" || item.rustType === "Object")
    )
      continue

    const isOptional =
      item.field.soe === "SOEOPTIONAL" || item.field.soe === "SOEDEFAULT"
    const getterName = toGetterName(item.field.sfName)
    const isFixedSizeOptional =
      item.kind === "field" && FIXED_SIZE_OPTIONAL_TYPES.has(item.rustType)
    ;(isOptional ? optionalGetters : requiredGetters).push({
      getterName,
      isFixedSizeOptional,
    })
  }

  const lines = [
    "#[cfg(test)]",
    "mod tests {",
    "    use super::*;",
    "    use crate::host::host_bindings_trait::MockHostBindings;",
    "    use crate::host::setup_mock;",
    "    use crate::objects::test_support::*;",
    "",
  ]

  // `read_all_fields`: smoke test that every getter (required + optional) succeeds when
  // every field is reported present.
  lines.push(
    "    #[test]",
    "    fn read_all_fields() {",
    "        let mut mock = MockHostBindings::new();",
    "        mock_all_fields_present(&mut mock);",
    "        let _guard = setup_mock(mock);",
    "",
    `        let obj = ${className}::new(0);`,
    "",
  )
  for (const { getterName } of [...requiredGetters, ...optionalGetters]) {
    lines.push(`        assert!(obj.${getterName}().is_ok());`)
  }
  lines.push("    }", "")

  // `optional_fields_none`: only emitted if there's at least one optional getter. When the
  // host reports FIELD_NOT_FOUND, fixed-size optional getters (match_result_code_with_
  // expected_bytes_optional) return `Ok(None)`; every other optional getter (variable-size
  // Amount/Issue/Blob fields, and the raw unmapped-type placeholder getters) routes
  // through match_result_code_optional / a `result_code > 0` check, both of which treat a
  // negative code -- including FIELD_NOT_FOUND -- as `Err`, not `Ok(None)`. So this test
  // only smoke-tests that the fixed-size getters decode `None` correctly; it deliberately
  // does not assert success for the variable-size/raw getters here, since FIELD_NOT_FOUND
  // is not how the host represents "absent" for those (a 0-byte success response is).
  const fixedSizeOptionalGetters = optionalGetters.filter(
    (g) => g.isFixedSizeOptional,
  )
  if (fixedSizeOptionalGetters.length > 0) {
    lines.push(
      "    #[test]",
      "    fn optional_fields_none() {",
      "        let mut mock = MockHostBindings::new();",
      "        mock_all_fields_not_found(&mut mock);",
      "        let _guard = setup_mock(mock);",
      "",
      `        let obj = ${className}::new(0);`,
      "",
    )
    for (const { getterName } of fixedSizeOptionalGetters) {
      lines.push(`        assert!(obj.${getterName}().unwrap().is_none());`)
    }
    lines.push("    }", "")
  }

  if (lines[lines.length - 1] === "") lines.pop()
  lines.push("}")
  return lines
}

// Renders one entry's complete file contents (banner + imports + slot trait
// + current trait + struct). For slot-only entries (see SLOT_ONLY_ENTRIES) the
// current-object trait is omitted -- those accessors are hand-written elsewhere.
function renderEntryFile(entry, resolved, fieldDocsForClass) {
  const slotOnly = SLOT_ONLY_ENTRIES.has(entry.className)
  const hasRawGetter = resolved.some((r) => r.kind === "raw")
  const hasAnyGetter = resolved.length > 0

  const slot = renderTrait(entry, resolved, fieldDocsForClass, {
    useCurrent: false,
  })
  const current = slotOnly
    ? null
    : renderTrait(entry, resolved, fieldDocsForClass, { useCurrent: true })
  const struct = renderStruct(entry)

  const usedRustTypes = new Set([
    ...slot.usedRustTypes,
    ...(current ? current.usedRustTypes : []),
  ])

  const importLines = buildImports(usedRustTypes, {
    needsSlotCommon: true,
    needsCurrentCommon: !slotOnly,
    needsSlotAccessor: hasAnyGetter,
    needsCurrentAccessor: hasAnyGetter && !slotOnly,
    needsResult: hasAnyGetter,
    hasRawGetter,
    hasRawRequired: slot.hasRawRequired || (current && current.hasRawRequired),
    hasRawOptional: slot.hasRawOptional || (current && current.hasRawOptional),
    needsSfield: hasAnyGetter,
  })

  const lines = [BANNER]
  if (hasRawGetter) {
    lines.push(
      `/// Placeholder buffer size for fields whose XRPL wire type has no genuine Rust`,
      `/// mapping yet (VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32, ...). Such getters return`,
      `/// raw, unparsed bytes; see the summary at the top of \`generated/mod.rs\`.`,
      `const ${RAW_UNMAPPED_FIELD_SIZE_CONST}: usize = ${RAW_UNMAPPED_FIELD_SIZE};`,
      "",
    )
  }
  lines.push(...importLines, "")
  lines.push(...slot.lines, "")
  if (current) lines.push(...current.lines, "")
  lines.push(...struct.lines, "")
  lines.push("", ...renderTestsBlock(entry, resolved))

  return lines.join("\n")
}

// Renders the `//!` module-doc header listing every field whose XRPL wire
// type has no genuine Rust mapping yet, grouped by wire type. Deterministic:
// wire types are sorted, and entries within each wire type are sorted too.
function renderUnmappedHeader(unmapped) {
  if (unmapped.length === 0) return []

  const byWireType = new Map()
  for (const { entry, sfName, wireType } of unmapped) {
    if (!byWireType.has(wireType)) byWireType.set(wireType, [])
    byWireType.get(wireType).push(`${entry}.${sfName}`)
  }

  const lines = [
    "//! Fields returned as raw bytes pending a typed Rust representation:",
  ]
  for (const wireType of [...byWireType.keys()].sort()) {
    const items = [...byWireType.get(wireType)].sort()
    lines.push(`//!   ${wireType}: ${items.join(", ")}`)
  }
  lines.push("")
  return lines
}

// Renders `objects/generated/mod.rs`: one `pub mod <snake_entry>;` per
// generated entry, plus a flat re-export of every public item.
function renderModFile(entries, unmapped) {
  const lines = [...renderUnmappedHeader(unmapped), BANNER]
  const modNames = entries.map((e) => snakeCase(e.className))
  for (const modName of [...modNames].sort()) {
    lines.push(`pub mod ${modName};`)
  }
  lines.push("")
  for (const entry of entries) {
    const modName = snakeCase(entry.className)
    const { className } = entry
    const items = SLOT_ONLY_ENTRIES.has(className)
      ? `${className}, ${className}Fields`
      : `${className}, ${className}Fields, Current${className}Fields`
    lines.push(`pub use ${modName}::{${items}};`)
  }
  return lines.join("\n") + "\n"
}

function snakeCase(className) {
  return toGetterName(className)
}

////////////////////////////////////////////////////////////////////////
//  Main
////////////////////////////////////////////////////////////////////////

async function main() {
  const rippledSource = process.argv[2]
  const outputDir = process.argv[3] || path.join(__dirname, "..")

  const [ledgerEntriesMacro, sfieldsMacro] = await Promise.all([
    read(rippledSource, "include/xrpl/protocol/detail/ledger_entries.macro"),
    read(rippledSource, "include/xrpl/protocol/detail/sfields.macro"),
  ])

  const entries = parseLedgerEntries(ledgerEntriesMacro)
  const sfieldTypes = parseSfields(sfieldsMacro)

  console.log(`Ledger entries generated: ${entries.length}`)

  let totalGetters = 0
  const unmapped = []

  const genDir = path.join(
    outputDir,
    "xrpl-common-stdlib/src/objects/generated",
  )
  await fs.mkdir(genDir, { recursive: true })

  // Prune stale generated files: any `.rs` already in genDir that this run will
  // not (re)write is an orphan from a removed or renamed ledger entry. Deleting
  // it here keeps the directory a faithful mirror of the current rippled source
  // -- otherwise a dropped entry would leave dead code on disk that `mod.rs` no
  // longer references (and that the drift check, comparing like-for-like, would
  // never flag). Sorted only for deterministic log output; deletion order is
  // irrelevant to the result.
  const expectedFiles = new Set([
    "mod.rs",
    ...entries.map((e) => `${snakeCase(e.className)}.rs`),
  ])
  const existingFiles = await fs.readdir(genDir)
  for (const name of existingFiles.sort()) {
    if (name.endsWith(".rs") && !expectedFiles.has(name)) {
      await fs.rm(path.join(genDir, name))
      console.log(`Pruned stale generated file: ${name}`)
    }
  }

  for (const entry of entries) {
    const resolved = resolveEntryFields(entry, sfieldTypes, unmapped)
    totalGetters += resolved.length

    const fieldDocsForClass = await fetchFieldDocs(entry.className)
    const contents = renderEntryFile(entry, resolved, fieldDocsForClass)

    const file = path.join(genDir, `${snakeCase(entry.className)}.rs`)
    await fs.writeFile(file, contents, "utf8")
  }

  const modFile = path.join(genDir, "mod.rs")
  await fs.writeFile(modFile, renderModFile(entries, unmapped), "utf8")

  console.log(`Total getters emitted (across all entries): ${totalGetters}`)
  console.log(
    `Total fields with no Rust type mapping (raw placeholder): ${unmapped.length}`,
  )
  if (unmapped.length > 0) {
    console.log("Unmapped fields (emitted as raw bytes):")
    for (const u of unmapped) {
      console.log(`  ${u.entry}.sf${u.sfName}: ${u.wireType}`)
    }
  }

  console.log("")
  console.log("🎨 Formatting generated output with rustfmt...")
  const allFiles = [
    modFile,
    ...entries.map((e) => path.join(genDir, `${snakeCase(e.className)}.rs`)),
  ]
  execFileSync("rustfmt", ["--edition", "2024", ...allFiles], {
    stdio: "inherit",
  })

  console.log("")
  console.log(
    "Note: generated files are consumed via 'pub mod generated;' in " +
      "xrpl-common-stdlib/src/objects/mod.rs.",
  )
}

main()
