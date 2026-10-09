//! Shared file-reading helpers for the rippled-source code generators.
//!
//! A "source" is either a local directory (e.g. `/path/to/rippled/`) or a
//! GitHub URL — either a bare repo like `https://github.com/XRPLF/rippled`
//! (which defaults to the `HEAD` ref) or a `https://github.com/XRPLF/rippled/tree/<ref>`
//! URL pinning a branch/tag/commit.

use std::error::Error;
use std::fs;
use std::path::Path;

/// Fetches `filename` from a rippled GitHub repo over HTTPS.
pub fn read_file_from_github(repo: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let url = github_raw_url(repo, filename);
    let response = ureq::get(&url)
        .call()
        .map_err(|e| format!("Error reading {url}: {e}"))?;
    response
        .into_string()
        .map_err(|e| format!("Error reading {url}: {e}").into())
}

/// Reads `filename` from a local directory on disk.
pub fn read_file(folder: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(folder).join(filename);
    fs::read_to_string(&path).map_err(|e| format!("File not found: {}, {e}", path.display()).into())
}

/// Reads `filename` relative to `source`, dispatching to a GitHub fetch or a
/// local file read depending on what `source` is. Source-ness is resolved per
/// call so callers can mix independent sources (e.g. a base branch and a
/// contract branch) in the same run.
pub fn read_source_file(source: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    if is_github_url(source) {
        read_file_from_github(source, filename)
    } else {
        read_file(source, filename)
    }
}

fn is_github_url(source: &str) -> bool {
    source.starts_with("https://github.com/") || source.starts_with("http://github.com/")
}

fn github_raw_url(repo: &str, filename: &str) -> String {
    let normalized = if !repo.contains("tree") {
        format!("{repo}/tree/HEAD")
    } else {
        repo.to_string()
    };

    let mut url = normalized
        .replace("github.com", "raw.githubusercontent.com")
        .replace("tree/", "");
    url.push('/');
    url.push_str(filename);

    if !url.starts_with("http") {
        url = format!("https://{url}");
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn is_github_url_detects_https_and_http() {
        assert!(is_github_url("https://github.com/XRPLF/rippled"));
        assert!(is_github_url("http://github.com/XRPLF/rippled"));
        assert!(!is_github_url("github.com/XRPLF/rippled"));
        assert!(!is_github_url("/path/to/rippled"));
        assert!(!is_github_url("./rippled"));
        assert!(!is_github_url(""));
    }

    #[test]
    fn github_raw_url_bare_repo_uses_head() {
        let url = github_raw_url("https://github.com/XRPLF/rippled", "src/foo.cpp");
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/XRPLF/rippled/HEAD/src/foo.cpp"
        );
    }

    #[test]
    fn github_raw_url_tree_ref_is_pinned() {
        let url = github_raw_url(
            "https://github.com/XRPLF/rippled/tree/develop",
            "src/foo.cpp",
        );
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/XRPLF/rippled/develop/src/foo.cpp"
        );
    }

    #[test]
    fn github_raw_url_adds_https_when_missing() {
        let url = github_raw_url("github.com/XRPLF/rippled", "src/foo.cpp");
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/XRPLF/rippled/HEAD/src/foo.cpp"
        );
    }

    #[test]
    fn read_file_returns_contents() {
        let dir = env::temp_dir().join("rippled_source_test_ok");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("hello.txt");
        fs::write(&path, "world").unwrap();

        let got = read_file(dir.to_str().unwrap(), "hello.txt").unwrap();
        assert_eq!(got, "world");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn read_file_missing_file_errors() {
        let dir = env::temp_dir().join("rippled_source_test_missing");
        fs::create_dir_all(&dir).unwrap();

        let err = read_file(dir.to_str().unwrap(), "nope.txt").unwrap_err();
        assert!(err.to_string().contains("File not found"));

        fs::remove_dir_all(&dir).unwrap();
    }
}
