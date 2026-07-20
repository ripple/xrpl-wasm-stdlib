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
    "Escrow-side fields are sourced from (and always trust) the escrow branch, so a rename there is picked up automatically. " +
      "The contract branch is only used for fields that don't exist on the escrow branch at all.",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")
const { readSourceFile: read } = require("./rippledSource")

function parseStypes(sfieldHeaderFile) {
  let stypeHits = [
    ...sfieldHeaderFile.matchAll(
      /^ *STYPE\(STI_([^ ]*?)[ \n]*,[ \n]*([0-9-]+)[ \n]*\)[ \n]*\\?$/gm,
    ),
  ]
  if (stypeHits.length === 0)
    stypeHits = [
      ...sfieldHeaderFile.matchAll(
        /^ *STI_([^ ]*?)[ \n]*=[ \n]*([0-9-]+)[ \n]*,?$/gm,
      ),
    ]
  const stypeMap = {}
  stypeHits.forEach(([_, key, value]) => {
    stypeMap[key] = value
  })
  return stypeMap
}

// Returns a Map from field name (without the "sf" prefix) to {xrplType, ordinal}.
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

// Escrow fields are authoritative (a rename/change there is always picked
// up); the contract branch only contributes fields escrow doesn't have at
// all. A field present in both with a *different* definition is a real
// divergence between the branches and needs a human, not a silent pick.
function mergeSfields(escrowFields, contractFields) {
  const merged = new Map(escrowFields)
  const addedFromContract = []
  let conflict = false
  for (const [name, def] of contractFields) {
    if (!merged.has(name)) {
      merged.set(name, def)
      addedFromContract.push(name)
      continue
    }
    const existing = merged.get(name)
    if (
      existing.xrplType !== def.xrplType ||
      existing.ordinal !== def.ordinal
    ) {
      console.error(
        `Conflict for field sf${name}: escrow branch defines (${existing.xrplType}, ${existing.ordinal}) but contract branch defines (${def.xrplType}, ${def.ordinal}). Resolve manually before regenerating.`,
      )
      conflict = true
    }
  }
  return { merged, addedFromContract, conflict }
}

async function main() {
  const escrowSource = process.argv[2]
  const contractSource = process.argv[3]

  const [escrowMacroFile, contractHeaderFile, contractMacroFile] =
    await Promise.all([
      read(escrowSource, "include/xrpl/protocol/detail/sfields.macro"),
      read(contractSource, "include/xrpl/protocol/SField.h"),
      read(contractSource, "include/xrpl/protocol/detail/sfields.macro"),
    ])

  let output = ""

  function addLine(line) {
    output += line + "\n"
  }

  // STI_* type codes are contract-only additions on top of escrow's set (no
  // escrow-side losses observed), so the contract branch alone is a safe,
  // strictly-superset source -- no merge needed here.
  const stypeMap = parseStypes(contractHeaderFile)

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

  ////////////////////////////////////////////////////////////////////////
  //  SField processing (escrow branch is authoritative; contract branch
  //  only contributes fields escrow doesn't have)
  ////////////////////////////////////////////////////////////////////////
  // NOTE: Output below replaces the constants section in sfield.rs
  // (starting after the impl blocks at line 52)

  addLine("pub const Invalid: SField<u8, -1> = SField::new();")
  addLine("pub const Generic: SField<u8, 0> = SField::new();")
  addLine("pub const hash: SField<u8, -1> = SField::new();")
  addLine("pub const index: SField<u8, 0> = SField::new();")
  addLine("")
  addLine("// Placeholder SField constants for array and object types")
  addLine(
    "// These types don't have FieldGetter implementations but are represented as SField<u8, CODE>",
  )

  const escrowFields = parseSfields(escrowMacroFile)
  const contractFields = parseSfields(contractMacroFile)
  const {
    merged: mergedFields,
    addedFromContract,
    conflict,
  } = mergeSfields(escrowFields, contractFields)

  console.log(
    `📝 SFields: ${escrowFields.size} from escrow branch, +${addedFromContract.length} contract-only additions${
      addedFromContract.length > 0
        ? ` (${addedFromContract.map((n) => "sf" + n).join(", ")})`
        : ""
    }, ${mergedFields.size} total`,
  )

  if (conflict) {
    console.error(
      "\n❌ One or more fields differ between the escrow and contract branches -- see above. Aborting without writing output.",
    )
    process.exit(1)
  }

  const sfieldEntries = [...mergedFields.entries()].map(([fieldName, def]) => ({
    fieldName,
    xrplType: def.xrplType,
    ordinal: def.ordinal,
  }))
  sfieldEntries.sort((a, b) => {
    const aValue = parseInt(stypeMap[a.xrplType]) * 2 ** 16 + a.ordinal
    const bValue = parseInt(stypeMap[b.xrplType]) * 2 ** 16 + b.ordinal
    return aValue - bValue // Ascending order
  })

  // Generate all field constants
  for (const entry of sfieldEntries) {
    const fieldName = entry.fieldName
    const xrplType = entry.xrplType
    const fieldCode = parseInt(stypeMap[xrplType]) * 2 ** 16 + entry.ordinal

    // Check for custom type override first, then fall back to typeMap
    let rustType = customFieldTypes[fieldName] || typeMap[xrplType]

    // Generate SField constant for all types
    if (rustType) {
      const line = `pub const ${fieldName}: SField<${rustType}, ${fieldCode}> = SField::new();`
      addLine(line)

      // Show custom type mappings
      if (customFieldTypes[fieldName]) {
        console.log(
          `  ✓ ${fieldName}: ${rustType} (custom mapping from ${xrplType})`,
        )
      }
    } else {
      // This should not happen if typeMap is complete
      console.warn(`Warning: No Rust type mapping for XRPL type: ${xrplType}`)
      addLine(
        `pub const ${fieldName}: SField<u8, ${fieldCode}> = SField::new();`,
      )
    }
  }

  ////////////////////////////////////////////////////////////////////////
  //  Serialized type processing (STI_* type codes)
  ////////////////////////////////////////////////////////////////////////
  // Sentinel type IDs that aren't real wire-format types: STI_UNKNOWN is
  // negative (doesn't fit in u8) and STI_NOTPRESENT is a "no value" marker.
  // Kept as comments for documentation, same as the hand-written original.
  const stiSentinelNames = new Set(["UNKNOWN", "NOTPRESENT"])
  // STI_TRANSACTION/LEDGERENTRY/VALIDATION/METADATA are pseudo-type IDs rippled
  // uses to tag whole serialized blobs (not real per-field STI values) and are
  // numbered >= 10000, well outside u8 range.

  let typeCodeOutput = ""
  function addTypeCodeLine(line) {
    typeCodeOutput += line + "\n"
  }

  addTypeCodeLine(
    "// Auto-generated by tools/generateSFields.js from rippled's include/xrpl/protocol/SField.h",
  )
  addTypeCodeLine(
    "// Do not hand-edit; re-run scripts/generate-sfields.sh instead.",
  )
  addTypeCodeLine("")

  const sortedStypes = Object.entries(stypeMap)
    .map(([name, value]) => [name, parseInt(value)])
    .sort((a, b) => a[1] - b[1])

  let previousValue = null
  let overflow = false
  for (const [name, value] of sortedStypes) {
    // Insert a blank line whenever there's a gap in the numeric sequence,
    // mirroring how the reserved/uncatalogued type IDs are skipped.
    if (previousValue !== null && value !== previousValue + 1) {
      addTypeCodeLine("")
    }
    previousValue = value

    if (stiSentinelNames.has(name) || value < 0 || value > 255) {
      if (value > 255 && !overflow) {
        addTypeCodeLine(
          "// The following type codes are outside the u8 range and are not valid for SField<u8, CODE>",
        )
        overflow = true
      }
      addTypeCodeLine(`// pub const STI_${name}: u8 = ${value};`)
    } else {
      addTypeCodeLine(`pub const STI_${name}: u8 = ${value};`)
    }
  }

  const typeCodesFile = path.join(
    __dirname,
    "../xrpl-wasm-stdlib/src/core/type_codes.rs",
  )
  try {
    await fs.writeFile(typeCodesFile, typeCodeOutput, "utf8")
    console.log("File written successfully to", typeCodesFile)
  } catch (err) {
    console.error("Error writing to file:", err)
  }

  const outputFile =
    process.argv.length == 5
      ? process.argv[4]
      : path.join(__dirname, "../xrpl-wasm-stdlib/src/sfield.rs")
  try {
    // Read existing file to preserve type definitions and impl blocks
    let existingContent = ""
    try {
      existingContent = await fs.readFile(outputFile, "utf8")
    } catch {
      // File doesn't exist yet, that's ok
    }

    // Find where the constants section starts (after impl blocks)
    // Look for the first "pub const Invalid" line (works for both old and new format)
    const constantsStartMarker = "pub const Invalid:"
    const existingConstantsStart = existingContent.indexOf(constantsStartMarker)

    let finalOutput
    if (existingConstantsStart !== -1) {
      // Extract the type definitions and impl blocks (everything before the constants)
      const typeDefinitions = existingContent.substring(
        0,
        existingConstantsStart,
      )
      // Combine type definitions with new constants
      finalOutput = typeDefinitions + output
    } else {
      // File doesn't have constants section yet, just use the new output
      finalOutput = output
    }

    await fs.writeFile(outputFile, finalOutput, "utf8")
    console.log("File written successfully to", outputFile)
  } catch (err) {
    console.error("Error writing to file:", err)
  }
}

main()
