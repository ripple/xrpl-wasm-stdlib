#!/bin/bash
# Sourced by the rippled-source generator scripts. Resolves which rippled
# commit/branch each generator should read.
#
# Defaults are PINNED to the commits CI tests against, both read from
# .github/workflows/test.yml (the single source of truth, same as
# docker-rippled.sh):
#   Smart Escrow source   (ripple/se/supported)   -> the tag of XRPLD_DOCKER_IMAGE (an xrpld commit sha)
#   Smart Contract source (xrplf/smart-contracts) -> XRPLD_CONTRACT_COMMIT
#
# Override either with an env var holding any git ref (branch, tag, or sha):
#   XRPLD_ESCROW_REF=ripple/se/supported XRPLD_CONTRACT_REF=xrplf/smart-contracts
# (This SDK supports both the Smart Escrow and Smart Contract xrpld sources; the
# generators merge fields/flags from the two.)
# That is what the scheduled upstream-drift workflow does to compare against
# branch HEADs without turning PR CI red when upstream moves.
#
# Requires REPO_ROOT to be set by the caller before sourcing.

XRPLD_PINS_WORKFLOW_FILE="${REPO_ROOT:?REPO_ROOT must be set before sourcing xrpld-pins.sh}/.github/workflows/test.yml"

_xrpld_pin_from_workflow() {
    # `|| true` so a missing key yields "" for the caller to report, instead of
    # `set -e` aborting silently inside the caller's assignment.
    { grep -m1 -E "^[[:space:]]*$1:" "$XRPLD_PINS_WORKFLOW_FILE" || true; } \
        | sed -E "s/^[[:space:]]*$1:[[:space:]]*//; s/[[:space:]]*(#.*)?$//"
}

xrpld_pinned_escrow_commit() {
    local image tag
    image="$(_xrpld_pin_from_workflow XRPLD_DOCKER_IMAGE)"
    if [[ -z "$image" ]]; then
        echo "❌ Could not read XRPLD_DOCKER_IMAGE from $XRPLD_PINS_WORKFLOW_FILE" >&2
        return 1
    fi
    tag="${image##*:}"
    if [[ ! "$tag" =~ ^[0-9a-f]{7,40}$ ]]; then
        echo "❌ XRPLD_DOCKER_IMAGE tag '$tag' is not a rippled commit sha; the generators need a commit to pin to" >&2
        return 1
    fi
    echo "$tag"
}

xrpld_pinned_contract_commit() {
    local sha
    sha="$(_xrpld_pin_from_workflow XRPLD_CONTRACT_COMMIT)"
    if [[ -z "$sha" ]]; then
        echo "❌ Could not read XRPLD_CONTRACT_COMMIT from $XRPLD_PINS_WORKFLOW_FILE" >&2
        return 1
    fi
    echo "$sha"
}

xrpld_escrow_source() {
    local ref
    ref="${XRPLD_ESCROW_REF:-$(xrpld_pinned_escrow_commit)}" || return 1
    echo "https://github.com/XRPLF/rippled/tree/$ref"
}

xrpld_contract_source() {
    local ref
    ref="${XRPLD_CONTRACT_REF:-$(xrpld_pinned_contract_commit)}" || return 1
    echo "https://github.com/XRPLF/rippled/tree/$ref"
}
