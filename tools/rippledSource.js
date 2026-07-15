// Shared file-reading helpers for the rippled-source code generators
// (generateSFlags.js, generateSFields.js, compareHostFunctions.js). A
// "source" is either a local directory or a GitHub URL (a bare repo, which
// defaults to HEAD, or a `.../tree/<ref>` URL).
const path = require("path")
const fs = require("fs/promises")

async function readFileFromGitHub(repo, filename) {
  if (!repo.includes("tree")) {
    repo += "/tree/HEAD"
  }
  let url = repo.replace("github.com", "raw.githubusercontent.com")
  url = url.replace("tree/", "")
  url += "/" + filename

  if (!url.startsWith("http")) {
    url = "https://" + url
  }

  try {
    const response = await fetch(url)
    if (!response.ok) {
      throw new Error(`${response.status} ${response.statusText}`)
    }
    return await response.text()
  } catch (e) {
    console.error(`Error reading ${url}: ${e.message}`)
    process.exit(1)
  }
}

async function readFile(folder, filename) {
  const filePath = path.join(folder, filename)
  try {
    return await fs.readFile(filePath, "utf-8")
  } catch (e) {
    throw new Error(`File not found: ${filePath}, ${e.message}`)
  }
}

// Reads `filename` relative to `source`. Source-ness is resolved per call
// (rather than once globally) so callers can mix independent sources (e.g.
// an escrow branch and a contract branch) in the same run.
async function readSourceFile(source, filename) {
  try {
    const url = new URL(source)
    if (url.hostname === "github.com") {
      return readFileFromGitHub(source, filename)
    }
  } catch {
    // Not a URL -- fall through to local file read.
  }
  return readFile(source, filename)
}

module.exports = { readFileFromGitHub, readFile, readSourceFile }
