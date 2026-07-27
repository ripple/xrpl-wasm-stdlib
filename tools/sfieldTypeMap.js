// Shared XRPL-type -> Rust-type mapping used by the sfield and ledger-object
// code generators. Extracted from generateSFields.js so both generators stay
// in sync automatically instead of maintaining two copies.

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
  FinishFunction: "WasmBlob",
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
//   yet representable in Rust (e.g. VECTOR256, XCHAIN_BRIDGE, NUMBER, INT32,
//   JSON) and callers must NOT fabricate a u8 getter for it.
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
}
