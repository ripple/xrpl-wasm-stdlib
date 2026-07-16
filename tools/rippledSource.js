// Shared file-reading helpers for the rippled-source code generators. A
// "source" is either a local directory or a GitHub URL (a bare repo, which
// defaults to HEAD, or a `.../tree/<ref>` URL).
const path = require("path")
const fs = require("fs/promises")

/**
 * Fetches `filename` from a rippled GitHub repo over HTTPS.
 *
 * @param {string} repo - A GitHub URL, either a bare repo (in which case the
 *   `HEAD` ref is used) or a `.../tree/<ref>` URL pinning a branch/tag/commit.
 * @param {string} filename - Path to the file relative to the repo root.
 * @returns {Promise<string>} The file's contents as UTF-8 text.
 *   Prints an error and exits the process on any fetch/HTTP failure.
 */
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

/**
 * Reads `filename` from a local directory on disk.
 *
 * @param {string} folder - Path to the directory containing the file.
 * @param {string} filename - Path to the file relative to `folder`.
 * @returns {Promise<string>} The file's contents as UTF-8 text.
 * @throws {Error} If the file cannot be read.
 */
async function readFile(folder, filename) {
  const filePath = path.join(folder, filename)
  try {
    return await fs.readFile(filePath, "utf-8")
  } catch (e) {
    throw new Error(`File not found: ${filePath}, ${e.message}`)
  }
}

/**
 * Reads `filename` relative to `source`, dispatching to a GitHub fetch or a
 * local file read depending on what `source` is. Source-ness is resolved per
 * call (rather than once globally) so callers can mix independent sources
 * (e.g. a base branch and a contract branch) in the same run.
 *
 * @param {string} source - A local directory path or a GitHub URL.
 * @param {string} filename - Path to the file relative to `source`.
 * @returns {Promise<string>} The file's contents as UTF-8 text.
 */
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
