//! Makes rippled's doc comments valid in our crate. rippled links to its own items —
//! `[`HostFunctions::get_tx_array_len`]`, `[`HASH_LEN`]`, `[`FloatOrdering`]` — which do not
//! exist here and would be `unresolved link` warnings under `cargo doc`.

use std::collections::HashMap;

/// Rewrites one doc line: `[`HostFunctions::<rust>`]` becomes `[`HostBindings::<wasm>`]`
/// when `<rust>` is a known declaration; every other `[`X`]` becomes plain `` `X` ``.
pub fn rewrite_doc_line(line: &str, rust_to_wasm: &HashMap<String, String>) -> String {
    const OPEN: &str = "[`";
    const CLOSE: &str = "`]";
    const METHOD_PREFIX: &str = "HostFunctions::";

    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find(OPEN) {
        let Some(len) = rest[start + OPEN.len()..].find(CLOSE) else {
            break;
        };
        let target = &rest[start + OPEN.len()..start + OPEN.len() + len];
        let after = start + OPEN.len() + len + CLOSE.len();
        if rest[after..].starts_with(['(', '[']) {
            // Real markdown link or reference link: keep verbatim.
            out.push_str(&rest[..after]);
            rest = &rest[after..];
            continue;
        }
        out.push_str(&rest[..start]);
        match target
            .strip_prefix(METHOD_PREFIX)
            .and_then(|m| rust_to_wasm.get(m))
        {
            Some(wasm) => out.push_str(&format!("[`HostBindings::{wasm}`]")),
            None => out.push_str(&format!("`{target}`")),
        }
        rest = &rest[start + OPEN.len() + len + CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> HashMap<String, String> {
        [
            ("get_ledger_sqn", "ldgr_index"),
            ("get_tx_array_len", "tx_arr_len"),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    }

    #[test]
    fn rewrites_method_links_to_our_trait_and_wasm_name() {
        assert_eq!(
            rewrite_doc_line(" as with [`HostFunctions::get_tx_array_len`].", &map()),
            " as with [`HostBindings::tx_arr_len`]."
        );
    }

    #[test]
    fn demotes_links_to_items_we_do_not_have() {
        assert_eq!(
            rewrite_doc_line(" the first [`HASH_LEN`] bytes of its SHA-512.", &map()),
            " the first `HASH_LEN` bytes of its SHA-512."
        );
    }

    #[test]
    fn demotes_method_links_we_cannot_resolve() {
        assert_eq!(
            rewrite_doc_line(" see [`HostFunctions::not_a_function`].", &map()),
            " see `HostFunctions::not_a_function`."
        );
    }

    #[test]
    fn handles_several_links_on_one_line() {
        assert_eq!(
            rewrite_doc_line(
                " [`HostFunctions::get_ledger_sqn`] and [`FloatOrdering`] here",
                &map()
            ),
            " [`HostBindings::ldgr_index`] and `FloatOrdering` here"
        );
    }

    #[test]
    fn leaves_inline_and_reference_links_verbatim() {
        for line in [
            " [`Foo`](https://x) tail",
            " [`HostFunctions::get_ledger_sqn`](url) tail",
            " [`Foo`][r] tail",
        ] {
            assert_eq!(rewrite_doc_line(line, &map()), line);
        }
    }

    #[test]
    fn leaves_lines_without_links_alone() {
        let line = " Plain `code` and **bold** and an unclosed [`bracket";
        assert_eq!(rewrite_doc_line(line, &map()), line);
    }
}
