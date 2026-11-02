#!/bin/bash
# Sourced by the generator scripts to implement their `--check` mode.
#
# check_generated <regen_fn> <hint> <path>...
#
#   regen_fn  name of a bash function that regenerates <path>... IN PLACE and
#             returns nonzero on failure (write it as an `&&` chain: it runs
#             with errexit suppressed).
#   hint      the command to print when drift is found, e.g.
#             "./scripts/generate-sfields.sh"
#   path...   files and/or directories the generator writes.
#
# Snapshots every path, runs regen_fn, diffs, then restores the snapshot no
# matter what -- a check must never leave the working tree dirty. Returns:
#   0  up to date
#   1  drift (unified diff printed)
#   2  regen_fn failed (its own error output is already on the terminal)

check_generated() {
    local regen_fn="$1" hint="$2"
    shift 2
    local paths=("$@")

    local snapshot
    snapshot="$(mktemp -d)"
    local i
    for i in "${!paths[@]}"; do
        cp -Rp "${paths[$i]}" "$snapshot/$i"
    done

    local status=0
    if ! "$regen_fn"; then
        status=2
    else
        for i in "${!paths[@]}"; do
            if ! diff -rq "$snapshot/$i" "${paths[$i]}" > /dev/null; then
                echo "❌ ${paths[$i]} is out of date"
                diff -ru "$snapshot/$i" "${paths[$i]}" || true
                status=1
            fi
        done
    fi

    for i in "${!paths[@]}"; do
        rm -rf "${paths[$i]}"
        cp -Rp "$snapshot/$i" "${paths[$i]}"
    done
    rm -rf "$snapshot"

    case "$status" in
        0) echo "✅ ${paths[*]} up to date" ;;
        1) echo ""
           echo "❌ Generated files are out of date."
           echo "💡 Run $hint and commit the result." ;;
        2) echo ""
           echo "❌ Generator failed; working tree restored. See errors above." ;;
    esac
    return "$status"
}
