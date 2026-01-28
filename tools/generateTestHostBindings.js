if (process.argv.length != 2) {
  console.error("Usage: " + process.argv[0] + " " + process.argv[1])
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Parse host bindings trait and update the other 3 files to match
////////////////////////////////////////////////////////////////////////
const path = require("path")
const fs = require("fs/promises")

const HOST_DIR = path.join(__dirname, "../xrpl-wasm-stdlib/src/host")

const FILES = {
  trait: "host_bindings_trait.rs",
  wasm: "host_bindings_wasm.rs",
  empty: "host_bindings_empty.rs",
  test: "host_bindings_test.rs",
}

async function readFile(filename) {
  const filePath = path.join(HOST_DIR, filename)
  try {
    return await fs.readFile(filePath, "utf-8")
  } catch (e) {
    throw new Error(`File not found: ${filePath}, ${e.message}`)
  }
}

async function writeFile(filename, content) {
  const filePath = path.join(HOST_DIR, filename)
  await fs.writeFile(filePath, content, "utf8")
  console.log(`  Updated ${filePath}`)
}

/**
 * Parse trait method signatures from host_bindings_trait.rs.
 * Extracts method name, parameters (excluding &self), and return type.
 * @param {string} content - The content of host_bindings_trait.rs
 * @returns {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>}
 */
function parseTraitMethods(content) {
  // Match unsafe fn declarations in the trait
  // Pattern: unsafe fn name(&self, params...) -> ReturnType;
  const regex =
    /unsafe fn ([A-Za-z0-9_]+)\s*\(\s*&self\s*(?:,\s*([^)]*))?\)\s*->\s*([A-Za-z0-9]+)\s*;/g

  const methods = []
  let match
  while ((match = regex.exec(content)) !== null) {
    const name = match[1]
    const paramsStr = match[2] || ""
    const returnType = match[3]

    // Parse parameters (excluding &self)
    const params = paramsStr
      .split(",")
      .map((p) => p.trim())
      .filter((p) => p.length > 0)
      .map((p) => {
        const parts = p.split(":")
        return {
          name: parts[0].trim(),
          type: parts.slice(1).join(":").trim(),
        }
      })

    methods.push({ name, params, returnType })
  }

  return methods
}

/**
 * Parse function signatures from export_host_functions! macro invocations.
 * Skips macro definitions (which contain $name:ident patterns) and only parses invocations.
 * Strips leading underscores from parameter names for consistent comparison.
 * @param {string} content - The file content containing export_host_functions! macro
 * @returns {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>}
 */
function parseExportMacro(content) {
  // Find all export_host_functions! macro invocations
  // Use a greedy match to get the largest block (the invocation, not the definition)
  const macroMatches = [
    ...content.matchAll(/export_host_functions!\s*\{([\s\S]*?)\n\}/g),
  ]
  if (macroMatches.length === 0) {
    return []
  }

  // Use the last (and typically largest) match which is the actual invocation
  // The macro definition contains rules like (@return_value) which we don't want
  let macroContent = ""
  for (const match of macroMatches) {
    // Skip macro definitions (they contain pattern matching syntax like $name:ident)
    if (!match[1].includes("$name:ident")) {
      macroContent = match[1]
    }
  }

  if (!macroContent) {
    return []
  }

  // Match fn declarations inside the macro
  const regex = /fn ([A-Za-z0-9_]+)\s*\(([^)]*)\)\s*->\s*([A-Za-z0-9]+)\s*;/g

  const functions = []
  let match
  while ((match = regex.exec(macroContent)) !== null) {
    const name = match[1]
    const paramsStr = match[2] || ""
    const returnType = match[3]

    // Parse parameters (strip leading underscore for comparison)
    const params = paramsStr
      .split(",")
      .map((p) => p.trim())
      .filter((p) => p.length > 0)
      .map((p) => {
        const parts = p.split(":")
        let paramName = parts[0].trim()
        // Remove leading underscore for comparison purposes
        if (paramName.startsWith("_")) {
          paramName = paramName.substring(1)
        }
        return {
          name: paramName,
          type: parts.slice(1).join(":").trim(),
        }
      })

    functions.push({ name, params, returnType })
  }

  return functions
}

/**
 * Parse impl HostBindings for WasmHostBindings methods.
 * Used for verification purposes (not currently called but kept for future use).
 * @param {string} content - The content of host_bindings_wasm.rs
 * @returns {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>}
 */
function parseWasmImpl(content) {
  // Find the impl block
  const implMatch = content.match(
    /impl HostBindings for WasmHostBindings\s*\{([\s\S]*?)\n\}/,
  )
  if (!implMatch) {
    return []
  }

  const implContent = implMatch[1]

  // Match unsafe fn declarations
  const regex =
    /unsafe fn ([A-Za-z0-9_]+)\s*\(\s*&self\s*(?:,\s*([^)]*))?\)\s*->\s*([A-Za-z0-9]+)/g

  const methods = []
  let match
  while ((match = regex.exec(implContent)) !== null) {
    const name = match[1]
    const paramsStr = match[2] || ""
    const returnType = match[3]

    const params = paramsStr
      .split(",")
      .map((p) => p.trim())
      .filter((p) => p.length > 0)
      .map((p) => {
        const parts = p.split(":")
        return {
          name: parts[0].trim(),
          type: parts.slice(1).join(":").trim(),
        }
      })

    methods.push({ name, params, returnType })
  }

  return methods
}

/**
 * Generate the export_host_functions! macro content for a file.
 * Groups functions by category and formats them with proper indentation.
 * @param {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>} methods - Parsed methods from trait
 * @param {boolean} withUnderscorePrefix - Whether to prefix parameter names with underscore (for unused params in stubs)
 * @returns {string} The formatted macro content
 */
function generateExportMacroContent(methods, withUnderscorePrefix = false) {
  const lines = []

  // Group functions by category based on name patterns
  const categories = [
    {
      name: "Host Function Category: ledger and transaction info",
      filter: (m) =>
        m.name.startsWith("get_ledger") ||
        m.name.startsWith("get_parent") ||
        m.name.startsWith("get_base") ||
        m.name.startsWith("get_tx") ||
        m.name.startsWith("get_current") ||
        m.name.startsWith("amendment") ||
        m.name.startsWith("cache"),
    },
    {
      name: "Host Function Category: update current ledger entry",
      filter: (m) => m.name === "update_data",
    },
    {
      name: "Host Function Category: hash and keylet computation",
      filter: (m) =>
        m.name.includes("keylet") ||
        m.name === "compute_sha512_half" ||
        m.name === "check_sig",
    },
    {
      name: "Host Function Category: NFT",
      filter: (m) => m.name.startsWith("get_nft"),
    },
    {
      name: "Host Function Category: FLOAT",
      filter: (m) => m.name.startsWith("float_"),
    },
    {
      name: "Host Function Category: TRACE",
      filter: (m) => m.name.startsWith("trace"),
    },
  ]

  const usedMethods = new Set()

  for (const category of categories) {
    const categoryMethods = methods.filter(
      (m) => category.filter(m) && !usedMethods.has(m.name),
    )
    if (categoryMethods.length === 0) continue

    lines.push(`    // ${category.name}`)

    for (const method of categoryMethods) {
      usedMethods.add(method.name)
      const params = method.params
        .map((p) => {
          const name = withUnderscorePrefix ? `_${p.name}` : p.name
          return `${name}: ${p.type}`
        })
        .join(", ")
      lines.push(`    fn ${method.name}(${params}) -> ${method.returnType};`)
    }
    lines.push("")
  }

  // Add any remaining methods not in categories
  const remaining = methods.filter((m) => !usedMethods.has(m.name))
  if (remaining.length > 0) {
    lines.push("    // Other functions")
    for (const method of remaining) {
      const params = method.params
        .map((p) => {
          const name = withUnderscorePrefix ? `_${p.name}` : p.name
          return `${name}: ${p.type}`
        })
        .join(", ")
      lines.push(`    fn ${method.name}(${params}) -> ${method.returnType};`)
    }
  }

  return lines.join("\n")
}

/**
 * Generate the extern "C" block content for host_bindings_wasm.rs.
 * Each function is declared with pub(super) visibility.
 * @param {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>} methods - Parsed methods from trait
 * @returns {string} The formatted extern block content
 */
function generateExternBlockContent(methods) {
  const lines = []

  for (const method of methods) {
    const params = method.params.map((p) => `${p.name}: ${p.type}`).join(", ")

    // Format based on parameter count for readability
    if (method.params.length <= 2) {
      lines.push(
        `        pub(super) fn ${method.name}(${params}) -> ${method.returnType};`,
      )
    } else {
      // Multi-line format for functions with many parameters
      lines.push(`        pub(super) fn ${method.name}(`)
      for (let i = 0; i < method.params.length; i++) {
        const p = method.params[i]
        const comma = i < method.params.length - 1 ? "," : ","
        lines.push(`            ${p.name}: ${p.type}${comma}`)
      }
      lines.push(`        ) -> ${method.returnType};`)
    }
  }

  return lines.join("\n")
}

/**
 * Generate the impl HostBindings for WasmHostBindings block content.
 * Each method calls the corresponding host_defined_functions function.
 * @param {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>} methods - Parsed methods from trait
 * @returns {string} The formatted impl block content
 */
function generateImplBlockContent(methods) {
  const lines = []

  for (const method of methods) {
    const params = method.params.map((p) => `${p.name}: ${p.type}`).join(", ")
    const paramNames = method.params.map((p) => p.name).join(", ")

    // Signature with &self
    const selfParams = params ? `&self, ${params}` : "&self"

    // Format based on parameter count for readability
    if (method.params.length <= 2) {
      lines.push(`    unsafe fn ${method.name}(${selfParams}) -> ${method.returnType} {`)
      if (paramNames) {
        lines.push(
          `        unsafe { host_defined_functions::${method.name}(${paramNames}) }`,
        )
      } else {
        lines.push(`        unsafe { host_defined_functions::${method.name}() }`)
      }
      lines.push(`    }`)
    } else {
      // Multi-line format for functions with many parameters
      lines.push(`    unsafe fn ${method.name}(`)
      lines.push(`        &self,`)
      for (let i = 0; i < method.params.length; i++) {
        const p = method.params[i]
        const comma = i < method.params.length - 1 ? "," : ","
        lines.push(`        ${p.name}: ${p.type}${comma}`)
      }
      lines.push(`    ) -> ${method.returnType} {`)
      lines.push(`        unsafe {`)
      lines.push(`            host_defined_functions::${method.name}(`)
      for (let i = 0; i < method.params.length; i++) {
        const p = method.params[i]
        const comma = i < method.params.length - 1 ? "," : ","
        lines.push(`                ${p.name}${comma}`)
      }
      lines.push(`            )`)
      lines.push(`        }`)
      lines.push(`    }`)
    }
    lines.push("")
  }

  return lines.join("\n")
}

/**
 * Update host_bindings_wasm.rs by replacing the extern block, impl block, and export macro.
 * @param {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>} methods - Parsed methods from trait
 */
async function updateWasmBindings(methods) {
  let content = await readFile(FILES.wasm)

  // 1. Update extern "C" block
  const externRegex =
    /(#\[link\(wasm_import_module = "host_lib"\)\]\s*unsafe extern "C" \{)([\s\S]*?)(\n    \})/
  const externMatch = content.match(externRegex)
  if (!externMatch) {
    throw new Error("Could not find extern block in host_bindings_wasm.rs")
  }
  const newExternContent = generateExternBlockContent(methods)
  content =
    content.substring(0, externMatch.index) +
    `${externMatch[1]}\n${newExternContent}\n    }` +
    content.substring(externMatch.index + externMatch[0].length)

  // 2. Update impl HostBindings for WasmHostBindings block
  const implRegex =
    /(\/\/\/ WASM implementation of HostBindings\.\nimpl HostBindings for WasmHostBindings \{)([\s\S]*?)(\n\})/
  const implMatch = content.match(implRegex)
  if (!implMatch) {
    throw new Error(
      "Could not find impl HostBindings for WasmHostBindings in host_bindings_wasm.rs",
    )
  }
  const newImplContent = generateImplBlockContent(methods)
  content =
    content.substring(0, implMatch.index) +
    `${implMatch[1]}\n${newImplContent}}` +
    content.substring(implMatch.index + implMatch[0].length)

  // 3. Update export_host_functions! macro (reuse existing logic)
  const macroRegex = /(export_host_functions!\s*\{)([\s\S]*?)(\n\})/g
  const matches = [...content.matchAll(macroRegex)]
  let targetMatch = null
  let targetIndex = -1
  for (const match of matches) {
    if (!match[2].includes("$name:ident")) {
      targetMatch = match
      targetIndex = match.index
    }
  }
  if (!targetMatch) {
    throw new Error(
      "No export_host_functions! invocation found in host_bindings_wasm.rs",
    )
  }
  const newMacroContent = generateExportMacroContent(methods, false)
  content =
    content.substring(0, targetIndex) +
    `${targetMatch[1]}\n${newMacroContent}\n}` +
    content.substring(targetIndex + targetMatch[0].length)

  await writeFile(FILES.wasm, content)
}

/**
 * Update a file by replacing the export_host_functions! macro content.
 * Finds the macro invocation (not definition) and replaces its content with generated signatures.
 * @param {string} filename - The file to update (relative to HOST_DIR)
 * @param {Array<{name: string, params: Array<{name: string, type: string}>, returnType: string}>} methods - Parsed methods from trait
 * @param {boolean} withUnderscorePrefix - Whether to prefix parameter names with underscore
 */
async function updateExportMacro(
  filename,
  methods,
  withUnderscorePrefix = false,
) {
  const content = await readFile(filename)

  // Find the last export_host_functions! macro invocation (not the definition)
  const macroRegex = /(export_host_functions!\s*\{)([\s\S]*?)(\n\})/g
  const matches = [...content.matchAll(macroRegex)]

  if (matches.length === 0) {
    throw new Error(`No export_host_functions! macro found in ${filename}`)
  }

  // Find the invocation (not the definition)
  let targetMatch = null
  let targetIndex = -1
  for (const match of matches) {
    if (!match[2].includes("$name:ident")) {
      targetMatch = match
      targetIndex = match.index
    }
  }

  if (!targetMatch) {
    throw new Error(`No export_host_functions! invocation found in ${filename}`)
  }

  const newMacroContent = generateExportMacroContent(
    methods,
    withUnderscorePrefix,
  )
  const newMacro = `${targetMatch[1]}\n${newMacroContent}\n}`

  const newContent =
    content.substring(0, targetIndex) +
    newMacro +
    content.substring(targetIndex + targetMatch[0].length)

  await writeFile(filename, newContent)
}

/**
 * Main entry point.
 * Reads the trait file as source of truth and updates the 3 derived files:
 * - host_bindings_wasm.rs: extern block, impl block, and export macro
 * - host_bindings_empty.rs: stub implementations with underscore-prefixed params
 * - host_bindings_test.rs: test implementations
 */
async function main() {
  console.log("Reading host bindings trait file (source of truth)...")

  const traitContent = await readFile(FILES.trait)

  // Parse trait methods (source of truth)
  const traitMethods = parseTraitMethods(traitContent)
  console.log(`  Found ${traitMethods.length} methods in ${FILES.trait}`)

  console.log("\nUpdating derived files...")

  // Update host_bindings_wasm.rs (extern block, impl block, and export macro)
  await updateWasmBindings(traitMethods)

  // Update host_bindings_empty.rs (with underscore prefix for unused params)
  await updateExportMacro(FILES.empty, traitMethods, true)

  // Update host_bindings_test.rs (without underscore prefix)
  await updateExportMacro(FILES.test, traitMethods, false)

  console.log(
    `\n✅ Successfully updated ${traitMethods.length} function signatures in 3 files.`,
  )
}

main().catch((e) => {
  console.error("Error:", e.message)
  process.exit(1)
})
