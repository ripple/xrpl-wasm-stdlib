if (process.argv.length != 4 && process.argv.length != 5) {
  console.error(
    "Usage: " +
      process.argv[0] +
      " " +
      process.argv[1] +
      " path/to/escrow/rippled path/to/contract/rippled [path/to/pipe/to]",
  )
  console.error(
    "Both rippled paths may be local dirs or GitHub URLs, e.g. https://github.com/XRPLF/rippled/tree/ripple/se/supported",
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
const { typeMap, customFieldTypes, parseSfields } = require("./sfieldTypeMap")

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
  addLine(
    "// Fields whose XRPL wire type has no decodable Rust type are still emitted, so",
  )
  addLine(
    "// their field code stays available: STI_ARRAY/STI_OBJECT as SField<Array, CODE>/",
  )
  addLine(
    "// SField<Object, CODE> (Locator navigation only), and every other unmapped wire",
  )
  addLine(
    "// type as SField<Unmapped, CODE>, which cannot be decoded at all -- see the",
  )
  addLine("// `Unmapped` docs above.")

  const escrowFields = parseSfields(escrowMacroFile)
  const contractFields = parseSfields(contractMacroFile)
  const {
    merged: mergedFields,
    addedFromContract,
    conflict,
  } = mergeSfields(escrowFields, contractFields)

  console.log(
    `SFields: ${escrowFields.size} from escrow branch, +${addedFromContract.length} contract-only additions${
      addedFromContract.length > 0
        ? ` (${addedFromContract.map((n) => "sf" + n).join(", ")})`
        : ""
    }, ${mergedFields.size} total`,
  )

  if (conflict) {
    console.error(
      "\nOne or more fields differ between the escrow and contract branches -- see above. Aborting without writing output.",
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

  // Generate all field constants.
  //
  // A field whose XRPL wire type has no Rust mapping still gets a constant, typed
  // `SField<Unmapped, CODE>`: the field code stays available to callers that only need the
  // code (a raw `le_field` read, a `Locator` path segment), while `Unmapped` implements no
  // decoder marker trait, so decoding one is a compile error rather than a read that
  // type-checks and fails at runtime. That's why no allowlist of "known untyped" wire types
  // is needed here -- a wire type rippled adds tomorrow lands on the same safe placeholder
  // instead of a wrong-typed `SField<u8, _>`. Unmapped types are reported below so a new one
  // is still visible; wire a real type in by adding it to `typeMap` in sfieldTypeMap.js.
  const unmappedTypes = new Map()
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
          ` ${fieldName}: ${rustType} (custom mapping from ${xrplType})`,
        )
      }
    } else {
      addLine(
        `pub const ${fieldName}: SField<Unmapped, ${fieldCode}> = SField::new();`,
      )
      if (!unmappedTypes.has(xrplType)) unmappedTypes.set(xrplType, [])
      unmappedTypes.get(xrplType).push(fieldName)
    }
  }

  if (unmappedTypes.size > 0) {
    console.log(
      `\nFields emitted as SField<Unmapped, _> (wire type has no Rust mapping yet):`,
    )
    for (const [xrplType, fields] of [...unmappedTypes].sort(([a], [b]) =>
      a.localeCompare(b),
    )) {
      console.log(
        `  STI_${xrplType}: ${fields.map((f) => "sf" + f).join(", ")}`,
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
    "../xrpl-common-stdlib/src/type_codes.rs",
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
      : path.join(__dirname, "../xrpl-common-stdlib/src/sfield.rs")
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
