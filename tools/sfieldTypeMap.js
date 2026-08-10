// Shared XRPL-type -> Rust-type mapping used by the sfield and ledger-object
// code generators. Extracted from generateSFields.js so both generators stay
// in sync automatically instead of maintaining two copies.

// Strip C-style comments (both `/* */` blocks and `//` lines) from a source
// string. Shared by every rippled-macro parser here.
function stripComments(text) {
  return text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\r\n]*/g, "")
}

// Parses an sfields.macro file into a Map<sfFieldName (without the "sf"
// prefix), {xrplType, ordinal}>. Shared by generateSFields.js and
// generateLedgerObjects.js so both read the field table the same way.
// e.g. `TYPED_SFIELD(sfAccount, ACCOUNT, 1)` -> map entry
//      "Account" => { xrplType: "ACCOUNT", ordinal: 1 }
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

// Rust blob-type aliases that live in `crate::types::blob`. Used by the
// ledger-object generator to route these types to the right import path.
const BLOB_TYPES = [
  "StandardBlob",
  "ConditionBlob",
  "FulfillmentBlob",
  "SignatureBlob",
  "UriBlob",
  "WasmBlob",
  "PublicKeyBlob",
]

// Map XRPL types to Rust types
// All types now have FieldGetter implementations
const typeMap = {
  UINT8: "u8",
  UINT16: "u16",
  UINT32: "u32",
  UINT64: "u64",
  UINT128: "Hash128",
  UINT160: "Hash160",
  UINT192: "Hash192",
  UINT256: "Hash256",
  AMOUNT: "Amount",
  NUMBER: "Number",
  ACCOUNT: "AccountID",
  VL: "StandardBlob",
  CURRENCY: "Currency",
  ISSUE: "Issue",
  ARRAY: "Array",
  OBJECT: "Object",
}

// Custom type overrides for specific field names
// These override the default type mapping from typeMap
const customFieldTypes = {
  TransactionType: "TransactionType",
  Condition: "ConditionBlob",
  Fulfillment: "FulfillmentBlob",
  Bytecode: "WasmBlob",
  PublicKey: "PublicKeyBlob",
  Domain: "UriBlob",
  MessageKey: "PublicKeyBlob",
  SigningPubKey: "PublicKeyBlob",
  TxnSignature: "SignatureBlob",
  URI: "UriBlob",
}

// Resolves the Rust type for a given sfield name + its XRPL wire type, giving
// customFieldTypes priority over the generic typeMap. Returns undefined if
// neither source has a mapping (caller decides how to handle that).
function resolveRustType(fieldName, xrplType) {
  return customFieldTypes[fieldName] || typeMap[xrplType]
}

// Like resolveRustType, but reports HOW the type was resolved so callers can
// distinguish a genuine mapping from a missing one. Returns:
//   { rustType, source: "custom" | "typeMap" | "none" }
// - "custom": a per-field-name override in customFieldTypes matched.
// - "typeMap": the XRPL wire type has a real Rust mapping (e.g. UINT8 -> u8).
// - "none": neither matched; rustType is undefined. The XRPL wire type is not
//   yet representable in Rust (e.g. VECTOR256, XCHAIN_BRIDGE, INT32, JSON) and
//   callers must NOT fabricate a u8 getter for it.
function resolveRustTypeDetailed(fieldName, xrplType) {
  if (Object.prototype.hasOwnProperty.call(customFieldTypes, fieldName)) {
    return { rustType: customFieldTypes[fieldName], source: "custom" }
  }
  if (Object.prototype.hasOwnProperty.call(typeMap, xrplType)) {
    return { rustType: typeMap[xrplType], source: "typeMap" }
  }
  return { rustType: undefined, source: "none" }
}

module.exports = {
  typeMap,
  customFieldTypes,
  resolveRustType,
  resolveRustTypeDetailed,
  stripComments,
  parseSfields,
  BLOB_TYPES,
}
