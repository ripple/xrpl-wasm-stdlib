if (process.argv.length !== 3) {
  console.error(
    "Usage: " + process.argv[0] + " " + process.argv[1] + " path/to/rippled",
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const { readFile, readSourceFile: read } = require("./rippledSource")

// rippled's WASM host ABI is declared in Rust (the `xrpl-host-functions` crate) and
// registered with the wasm engine in `xrpl-wasm-vm`. The two files are correlated by
// the `HostFunctionSpec::<Variant>` identifier, which is the PascalCase form of the
// declaration's Rust function name.
const ABI_DECLARATION_FILE = "crates/xrpl-host-functions/src/lib.rs"
const ABI_REGISTRATION_FILE = "crates/xrpl-wasm-vm/src/register.rs"

function areListsEqual(a, b) {
  if (a.length !== b.length) return false
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) {
      return false
    }
  }
  return true
}

function snakeToPascal(name) {
  return name
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("")
}

/**
 * Parses the `host_functions! { ... }` macro block in `crates/xrpl-host-functions/src/lib.rs`.
 *
 * Only the `#[wasm_name = "..."]` attribute is taken from here: it is the literal import
 * name the guest links against. The declaration's parameter types are high-level Rust
 * (`&[u8]`, `i32`, ...) and are *not* a reliable guide to the wire signature -- e.g. a
 * `seq: i32` is passed on the wire as a (ptr, len) byte region -- so params and return
 * type come from the registration file instead.
 *
 * Returns a Map keyed by `HostFunctionSpec` variant name (e.g. `GetLedgerSqn`) whose
 * values are `{ rustName, wasmName }`.
 */
function parseAbiDeclarations(source) {
  const blockStart = source.search(/^host_functions!\s*\{/m)
  if (blockStart < 0) {
    console.error(
      `Could not find the host_functions! block in ${ABI_DECLARATION_FILE}`,
    )
    process.exit(1)
  }
  // The macro block is at the top level, so its closing brace is the first
  // `}` at column 0 after the opening line.
  const blockEnd = source.indexOf("\n}", blockStart)
  if (blockEnd < 0) {
    console.error(
      `Could not find the end of the host_functions! block in ${ABI_DECLARATION_FILE}`,
    )
    process.exit(1)
  }
  const block = source
    .slice(blockStart, blockEnd)
    // Strip comments so `fn`/`#[...]` inside prose can't confuse the parser.
    .replace(/^\s*\/\/.*$/gm, "")

  const declarations = new Map()
  const declRegex =
    /((?:\s*#\[[^\]]*\])+)\s*fn\s+([a-z0-9_]+)\s*\(([^)]*)\)\s*->\s*HostResult<([^>]*)>\s*;/g
  for (const hit of block.matchAll(declRegex)) {
    const [, attrs, rustName] = hit
    const wasmNameMatch = attrs.match(/#\[wasm_name\s*=\s*"([A-Za-z0-9_]+)"\]/)
    if (!wasmNameMatch) {
      console.error(
        `${ABI_DECLARATION_FILE}: fn ${rustName} has no #[wasm_name = "..."] attribute`,
      )
      process.exit(1)
    }

    const variant = snakeToPascal(rustName)
    if (declarations.has(variant)) {
      console.error(
        `${ABI_DECLARATION_FILE}: duplicate host function declaration ${rustName}`,
      )
      process.exit(1)
    }
    declarations.set(variant, { rustName, wasmName: wasmNameMatch[1] })
  }
  return declarations
}

/**
 * Parses the `match op { HostFunctionSpec::X => linker.func_wrap(...) }` arms in
 * `crates/xrpl-wasm-vm/src/register.rs`. Each arm's closure header is the real
 * wasm-level import signature: flat `i32`/`i64` params (after the `caller`) and a
 * `Result<i32, _>` or `Result<(), _>` return.
 *
 * Returns a Map keyed by `HostFunctionSpec` variant name whose values are
 * `{ params: string[], return: string }` using wasm value types (`i32`, `i64`, `void`).
 */
function parseAbiRegistrations(source) {
  const armRegex =
    /HostFunctionSpec::([A-Za-z0-9]+)\s*=>\s*linker\.func_wrap\(/g
  const armCount = [...source.matchAll(armRegex)].length

  const closureRegex =
    /HostFunctionSpec::([A-Za-z0-9]+)\s*=>\s*linker\.func_wrap\(\s*HOST_MODULE,\s*op\.wasm_name\(\),\s*\|\s*mut caller:\s*Caller<'_,\s*VmState<'_>>((?:\s*,\s*[a-z0-9_]+\s*:\s*[a-z0-9]+)*)\s*\|\s*->\s*Result<(\(\)|[a-z0-9]+),\s*wasmi::Error>/g
  const registrations = new Map()
  for (const hit of source.matchAll(closureRegex)) {
    const [, variant, rawParams, rawReturn] = hit
    const params = rawParams
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s.length > 0)
      .map((s) => s.slice(s.indexOf(":") + 1).trim())
    if (registrations.has(variant)) {
      console.error(
        `${ABI_REGISTRATION_FILE}: HostFunctionSpec::${variant} is registered twice`,
      )
      process.exit(1)
    }
    registrations.set(variant, {
      params,
      return: rawReturn === "()" ? "void" : rawReturn,
    })
  }

  if (registrations.size !== armCount) {
    console.error(
      `${ABI_REGISTRATION_FILE}: found ${armCount} HostFunctionSpec arms but could only parse ${registrations.size} closure signatures; the file's shape has changed and this parser needs updating`,
    )
    process.exit(1)
  }
  return registrations
}

async function main() {
  const abiDeclarationFile = await read(process.argv[2], ABI_DECLARATION_FILE)
  const abiRegistrationFile = await read(process.argv[2], ABI_REGISTRATION_FILE)

  const declarations = parseAbiDeclarations(abiDeclarationFile)
  console.log(
    `\n📝 ${ABI_DECLARATION_FILE}: parsed ${declarations.size} host function declarations`,
  )
  const registrations = parseAbiRegistrations(abiRegistrationFile)
  console.log(
    `📝 ${ABI_REGISTRATION_FILE}: parsed ${registrations.size} host function registrations`,
  )

  const declaredVariants = [...declarations.keys()].sort()
  const registeredVariants = [...registrations.keys()].sort()
  if (!areListsEqual(declaredVariants, registeredVariants)) {
    console.error(
      "rippled's host function declarations and registrations do not match!",
    )
    const unregistered = declaredVariants.filter((v) => !registrations.has(v))
    const undeclared = registeredVariants.filter((v) => !declarations.has(v))
    if (unregistered.length > 0)
      console.error(
        `Declared in ${ABI_DECLARATION_FILE} but not registered:`,
        "\x1b[31m" + unregistered.join(", ") + "\x1b[0m",
      )
    if (undeclared.length > 0)
      console.error(
        `Registered in ${ABI_REGISTRATION_FILE} but not declared:`,
        "\x1b[31m" + undeclared.join(", ") + "\x1b[0m",
      )
    process.exit(1)
  }

  const rippledHostFunctions = declaredVariants
    .map((variant) => {
      const reg = registrations.get(variant)
      return {
        name: declarations.get(variant).wasmName,
        return: reg.return,
        params: reg.params,
      }
    })
    .sort((a, b) => a.name.localeCompare(b.name))

  const duplicateWasmNames = rippledHostFunctions.filter(
    (f, i) => i > 0 && rippledHostFunctions[i - 1].name === f.name,
  )
  if (duplicateWasmNames.length > 0) {
    console.error(
      "rippled declares the same wasm_name more than once:",
      "\x1b[31m" + duplicateWasmNames.map((f) => f.name).join(", ") + "\x1b[0m",
    )
    process.exit(1)
  }

  // Our FFI bindings are compared at the wasm-value level, which is what actually has
  // to agree for the import to link: pointers and lengths are both `i32` on wasm32.
  const paramTranslation = {
    i32: "i32",
    u32: "i32",
    usize: "i32",
    i64: "i64",
    "*const u8": "i32",
    "*mut u8": "i32",
  }

  function translateParamType(param) {
    if (param in paramTranslation) {
      return paramTranslation[param]
    }
    console.error(`Unknown parameter type: ${param}`)
    process.exit(1)
  }

  // Fire-and-forget host functions (the trace family) have no return value. Rust spells that
  // as either an omitted `->` or an explicit `-> ()`; rippled registers them as `Result<(), _>`.
  function translateReturnType(ret) {
    if (ret === undefined || ret === "()") {
      return "void"
    }
    return translateParamType(ret)
  }

  function checkHits(fileTitle, rustHostFunctions) {
    console.log(`\n🔍 Comparing ${fileTitle} with rippled host functions...`)
    console.log(`   Found ${rustHostFunctions.length} Rust functions`)
    console.log(`   Found ${rippledHostFunctions.length} rippled functions`)

    if (
      !areListsEqual(
        rustHostFunctions.map((f) => f.name),
        rippledHostFunctions.map((f) => f.name),
      )
    ) {
      console.error(
        `\n❌ ${fileTitle}: Rust Host Functions and rippled Host Functions do not match!`,
      )
      const rustMissing = rippledHostFunctions.filter(
        (f) => !rustHostFunctions.some((rf) => rf.name === f.name),
      )
      const rippledMissing = rustHostFunctions.filter(
        (f) => !rippledHostFunctions.some((rf) => rf.name === f.name),
      )
      if (rustMissing.length > 0)
        console.error(
          `   Missing Rust Host Functions in ${fileTitle}:`,
          "\x1b[31m" + rustMissing.map((f) => f.name).join(", ") + "\x1b[0m",
        )
      if (rippledMissing.length > 0)
        console.error(
          `   Missing rippled Host Functions (extra in ${fileTitle}):`,
          "\x1b[31m" + rippledMissing.map((f) => f.name).join(", ") + "\x1b[0m",
        )
      process.exit(1)
    }

    let hasError = false
    rustHostFunctions.forEach((hit, index) => {
      const rippledHit = rippledHostFunctions[index]
      if (hit.name !== rippledHit.name) {
        console.error(
          `Rust Host Function name mismatch in ${fileTitle}: ${hit.name} !== ${rippledHit.name}`,
        )
        hasError = true
      } else if (hit.return !== rippledHit.return) {
        console.error(
          `Rust Host Function return type mismatch in ${fileTitle} for ${hit.name}: ${hit.return} !== ${rippledHit.return}`,
        )
        hasError = true
      } else if (hit.params.length !== rippledHit.params.length) {
        console.error(
          `Rust Host Function parameter count mismatch in ${fileTitle} for ${hit.name}: ${hit.params.length} !== ${rippledHit.params.length} (${hit.params.join(", ")}) !== (${rippledHit.params.join(", ")})`,
        )
        hasError = true
      } else {
        hit.params.forEach((param, paramIndex) => {
          if (param !== rippledHit.params[paramIndex]) {
            console.error(
              `Rust Host Function parameter type mismatch in ${fileTitle} for ${hit.name}, parameter ${paramIndex}: ${param} !== ${rippledHit.params[paramIndex]}`,
            )
            hasError = true
          }
        })
      }
    })
    if (hasError) {
      process.exit(1)
    }
  }

  // host_bindings_trait.rs
  {
    const rustHostFunctionFile = await readFile(
      __dirname,
      "../xrpl-common-stdlib/src/host/host_bindings_trait.rs",
    )

    // Match multiline function declarations - need to match across newlines
    const regex =
      /unsafe fn ([A-Za-z0-9_]+)\(\s*&self(?:,\s*([^)]*))?\s*\)\s*(?:->\s*([A-Za-z0-9]+|\(\)))?\s*;/gm
    let rustHits = [...rustHostFunctionFile.matchAll(regex)]
    console.log(
      `\n📝 host_bindings_trait.rs: Regex matched ${rustHits.length} functions`,
    )
    if (rustHits.length < 10) {
      console.log(
        `   Matched functions: ${rustHits.map((h) => h[1]).join(", ")}`,
      )
    }

    const rustFuncs = rustHits.map((hit) => {
      const params = hit[2]
        ? hit[2]
            .replace(/\n/g, " ") // Replace newlines with spaces
            .trim()
            .split(",")
            .map((s) => s.trim())
            .filter((s) => s.length > 0)
            .map((s) => s.split(":")[1].trim())
        : []
      return [hit[1], hit[3], params]
    })
    const rustHostFunctions = rustFuncs
      .map((hit) => {
        return {
          name: hit[0],
          return: translateReturnType(hit[1]),
          params: hit[2].map(translateParamType),
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings_trait.rs", rustHostFunctions)
  }

  // host_bindings_wasm.rs
  {
    const rustHostfunctionFile = await readFile(
      __dirname,
      "../xrpl-common-stdlib/src/host/host_bindings_wasm.rs",
    )

    // Match multiline function declarations
    const regex =
      /pub\(super\) fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*(?:->\s*([A-Za-z0-9]+|\(\)))?\s*;/gm
    let rustHits = [...rustHostfunctionFile.matchAll(regex)]
    console.log(
      `\n📝 host_bindings_wasm.rs: Regex matched ${rustHits.length} functions`,
    )
    if (rustHits.length < 10) {
      console.log(
        `   Matched functions: ${rustHits.map((h) => h[1]).join(", ")}`,
      )
    }

    const rustFuncs = rustHits.map((hit) => {
      const params = hit[2]
        .replace(/\n/g, " ") // Replace newlines with spaces
        .trim()
        .split(",")
        .map((s) => s.trim())
        .filter((s) => s.length > 0)
        .map((s) => s.split(":")[1].trim())
      return [hit[1], hit[3], params]
    })
    const rustHostFunctions = rustFuncs
      .map((hit) => {
        return {
          name: hit[0],
          return: translateReturnType(hit[1]),
          params: hit[2].map(translateParamType),
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings_wasm.rs", rustHostFunctions)
  }

  // host_bindings_test.rs
  {
    const rustHostFunctionFile = await readFile(
      __dirname,
      "../xrpl-common-stdlib/src/host/host_bindings_test.rs",
    )

    // Extract only the export_host_functions! macro invocation (not the definition)
    // Look for the pattern that starts with a comment about generating stub functions
    const macroMatch = rustHostFunctionFile.match(
      /\s*export_host_functions!\s*\{([\s\S]*?)\n\}/,
    )
    if (!macroMatch) {
      console.error(
        "Could not find export_host_functions! macro invocation in host_bindings_test.rs",
      )
      process.exit(1)
    }
    const macroContent = macroMatch[1]

    // Match multiline function declarations (inside export_host_functions! macro)
    const regex =
      /fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*(?:->\s*([A-Za-z0-9]+|\(\)))?\s*;/gm
    let rustTestHits = [...macroContent.matchAll(regex)]
    console.log(
      `\n📝 host_bindings_test.rs: Regex matched ${rustTestHits.length} functions`,
    )
    if (rustTestHits.length < 10) {
      console.log(
        `   Matched functions: ${rustTestHits.map((h) => h[1]).join(", ")}`,
      )
    }

    const rustTestFuncs = rustTestHits.map((hit) => {
      const params = hit[2]
        .replace(/\n/g, " ") // Replace newlines with spaces
        .trim()
        .split(",")
        .map((s) => s.trim())
        .filter((s) => s.length > 0)
        .map((s) => s.split(":")[1].trim())
      return [hit[1], hit[3], params]
    })
    const rustTestHostFunctions = rustTestFuncs
      .map((hit) => {
        return {
          name: hit[0],
          return: translateReturnType(hit[1]),
          params: hit[2].map(translateParamType),
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings_test.rs", rustTestHostFunctions)
  }

  // host_bindings_empty.rs
  {
    const rustHostFunctionFile = await readFile(
      __dirname,
      "../xrpl-common-stdlib/src/host/host_bindings_empty.rs",
    )

    // Extract only the export_host_functions! macro invocation (not the definition)
    // Look for the pattern that starts with a comment about generating stub functions
    const macroMatch = rustHostFunctionFile.match(
      /^\s*export_host_functions!\s*\{([\s\S]*?)\n\}/m,
    )
    if (!macroMatch) {
      console.error(
        "Could not find export_host_functions! macro invocation in host_bindings_empty.rs",
      )
      process.exit(1)
    }
    const macroContent = macroMatch[1]

    // Match multiline function declarations (inside export_host_functions! macro)
    const regex =
      /fn ([A-Za-z0-9_]+)\(\s*([^)]*)\s*\)\s*(?:->\s*([A-Za-z0-9]+|\(\)))?\s*;/gm
    // The unit-returning trace stubs can't go through the macro (it coerces the last parameter
    // to i32 as the return value), so they're written out longhand after the invocation.
    const longhandRegex =
      /^pub unsafe fn ([A-Za-z0-9_]+)\(\s*([^)]*?)\s*,?\s*\)\s*(?:->\s*([A-Za-z0-9]+|\(\)))?\s*\{/gm
    let rustEmptyHits = [
      ...macroContent.matchAll(regex),
      ...rustHostFunctionFile.matchAll(longhandRegex),
    ]
    console.log(
      `\n📝 host_bindings_empty.rs: Regex matched ${rustEmptyHits.length} functions`,
    )

    const rustEmptyFuncs = rustEmptyHits.map((hit) => {
      const params = hit[2]
        .replace(/\n/g, " ") // Replace newlines with spaces
        .trim()
        .split(",")
        .map((s) => s.trim())
        .filter((s) => s.length > 0)
        .map((s) => s.split(":")[1].trim())
      return [hit[1], hit[3], params]
    })
    const rustEmptyHostFunctions = rustEmptyFuncs
      .map((hit) => {
        return {
          name: hit[0],
          return: translateReturnType(hit[1]),
          params: hit[2].map(translateParamType),
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))
    checkHits("host_bindings_empty.rs", rustEmptyHostFunctions)
  }

  console.log(
    "All host functions match between the Rust bindings and rippled's host ABI.",
  )
}

main()
