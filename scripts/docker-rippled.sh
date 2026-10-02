#!/bin/bash
# Start/stop a local rippled (xrpld) node in Docker, pinned to the same image CI uses.
# This keeps local integration tests in sync with the field/transaction definitions
# that the npm packages in package.json were generated against.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

CONTAINER_NAME="xrpld-service"
WORKFLOW_FILE=".github/workflows/test.yml"
RIPPLED_WS_PORT=6006

usage() {
    echo "Usage: $0 {start|stop|status|logs|verify}"
    exit 1
}

require_docker() {
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker not found. Install Docker, or set NO_DOCKER=true / DEVNET=true to skip it." >&2
        exit 1
    fi
    if ! docker info &> /dev/null; then
        echo "❌ Docker daemon not reachable. Start Docker Desktop (or dockerd), or set NO_DOCKER=true / DEVNET=true to skip it." >&2
        exit 1
    fi
}

get_image() {
    # Single source of truth: the image tag CI pins in test.yml. Reading it here
    # means this script can never drift from what CI actually verified against.
    # `|| true` so a missing key yields "" for the caller to report, instead of
    # `set -e` aborting silently inside the caller's assignment.
    { grep -m1 'XRPLD_DOCKER_IMAGE:' "$WORKFLOW_FILE" || true; } | sed -E 's/^[[:space:]]*XRPLD_DOCKER_IMAGE:[[:space:]]*//'
}

is_healthy() {
    # Exact match: a bare `grep healthy` would also match the "unhealthy" state.
    [[ "$(docker inspect --format='{{.State.Health.Status}}' "$CONTAINER_NAME" 2>/dev/null)" == "healthy" ]]
}

# True if something is already accepting connections on the rippled WS port,
# whether that's our own container, a manually-run rippled, or a native build.
# Checked with a bare TCP dial (bash's /dev/tcp) so this works even without Docker installed.
is_port_open() {
    (: < "/dev/tcp/127.0.0.1/$RIPPLED_WS_PORT") 2>/dev/null
}

# Prints the commit rippled on the WS port was built from (server_info's `git.hash`),
# or nothing if it can't be read: `git` is only returned on admin connections, and
# Node's built-in WebSocket needs Node 22+. Uses no npm packages.
get_running_hash() {
    command -v node &> /dev/null || return 0
    node -e '
        if (typeof WebSocket === "undefined") process.exit(1);
        setTimeout(() => process.exit(1), 10000);
        const ws = new WebSocket(`ws://127.0.0.1:${process.argv[1]}`);
        ws.onopen = () => ws.send(JSON.stringify({ command: "server_info" }));
        ws.onmessage = (e) => {
            console.log(JSON.parse(e.data).result?.info?.git?.hash ?? "");
            process.exit(0);
        };
        ws.onerror = () => process.exit(1);
    ' "$RIPPLED_WS_PORT" 2>/dev/null || true
}

# Fail if the rippled answering on the WS port isn't built from the commit CI pins.
# The image check in start() only covers our own container; this also covers a
# rippled we didn't start (a native build, another container). Skipped - not failed -
# when the pinned tag isn't a commit hash or the running build can't be read.
verify_build() {
    if [[ "${SKIP_RIPPLED_BUILD_CHECK:-false}" == "true" ]]; then
        echo "⚠️  SKIP_RIPPLED_BUILD_CHECK set - not verifying the rippled build."
        return 0
    fi

    local image expected actual
    image="$(get_image)"
    expected="${image##*:}"
    if [[ ! "$expected" =~ ^[0-9a-f]{40}$ ]]; then
        echo "ℹ️  Pinned tag '$expected' isn't a commit hash - skipping rippled build check."
        return 0
    fi

    actual="$(get_running_hash)"
    if [[ -z "$actual" ]]; then
        echo "⚠️  Couldn't read git.hash from server_info on ws://localhost:$RIPPLED_WS_PORT (needs Node 22+ and an admin connection) - skipping rippled build check."
        return 0
    fi
    if [[ "$actual" != "$expected" ]]; then
        echo "❌ rippled on ws://localhost:$RIPPLED_WS_PORT is built from $actual, but CI pins $expected ($image)." >&2
        echo "   Stop it and rerun to use the pinned Docker image, or set SKIP_RIPPLED_BUILD_CHECK=true to test against it anyway." >&2
        exit 1
    fi
    echo "✅ rippled build matches the pinned commit ($expected)."
}

start() {
    local image
    image="$(get_image)"
    if [[ -z "$image" ]]; then
        echo "❌ Could not read XRPLD_DOCKER_IMAGE from $WORKFLOW_FILE" >&2
        exit 1
    fi

    # Only reuse the container if it runs the image CI currently pins. Otherwise a
    # container left over from before an image bump would silently test against the
    # old rippled. Check the image whatever the health state, and remove a stale
    # container before the port check below: one that's still starting (or unhealthy)
    # can already have the port open and would be reused via that fallback.
    local running_image
    running_image="$(docker inspect --format='{{.Config.Image}}' "$CONTAINER_NAME" 2>/dev/null || true)"
    if [[ -n "$running_image" && "$running_image" != "$image" ]]; then
        echo "♻️  $CONTAINER_NAME is running $running_image but CI pins $image - restarting."
        docker rm -f "$CONTAINER_NAME" &> /dev/null || true
    elif is_healthy; then
        echo "✅ $CONTAINER_NAME is already running and healthy."
        verify_build
        return 0
    fi

    if is_port_open; then
        echo "✅ Something is already listening on ws://localhost:$RIPPLED_WS_PORT - assuming rippled is already running; skipping Docker."
        verify_build
        return 0
    fi

    require_docker

    # A stopped/unhealthy container from a previous run shouldn't block a fresh start.
    docker rm -f "$CONTAINER_NAME" &> /dev/null || true

    echo "🐳 Starting $CONTAINER_NAME from $image ..."
    docker run --detach --rm \
        -p 5005:5005 -p 6006:6006 \
        --volume "$REPO_ROOT/.ci-config/:/etc/xrpld/" \
        --name "$CONTAINER_NAME" \
        --health-cmd="xrpld server_info || exit 1" \
        --health-interval=5s --health-retries=10 --health-timeout=2s \
        --entrypoint bash "$image" -c "xrpld -a" > /dev/null

    echo "⏳ Waiting for $CONTAINER_NAME to be healthy..."
    # Avoid GNU coreutils' `timeout`, which macOS doesn't ship by default.
    local elapsed=0
    until is_healthy; do
        if (( elapsed >= 120 )); then
            echo "❌ $CONTAINER_NAME did not become healthy in time." >&2
            docker logs --tail 50 "$CONTAINER_NAME" 2>&1 || true
            exit 1
        fi
        sleep 5
        elapsed=$(( elapsed + 5 ))
    done
    echo "✅ $CONTAINER_NAME is healthy."
    verify_build
}

stop() {
    docker stop "$CONTAINER_NAME" &> /dev/null || true
    echo "🛑 $CONTAINER_NAME stopped."
}

status() {
    if is_healthy; then
        echo "✅ $CONTAINER_NAME is running and healthy."
    else
        echo "⚫ $CONTAINER_NAME is not running."
    fi
}

logs() {
    echo "=== Docker container logs ==="
    docker logs "$CONTAINER_NAME" 2>&1 || echo "Could not get logs"
    echo "=== Docker container status ==="
    docker inspect "$CONTAINER_NAME" 2>&1 || echo "Could not inspect container"
}

case "${1:-}" in
    start) start ;;
    stop) stop ;;
    status) status ;;
    logs) logs ;;
    verify) verify_build ;;
    *) usage ;;
esac
