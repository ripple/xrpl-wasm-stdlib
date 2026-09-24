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
    echo "Usage: $0 {start|stop|status|logs}"
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
    grep -m1 'XRPLD_DOCKER_IMAGE:' "$WORKFLOW_FILE" | sed -E 's/^[[:space:]]*XRPLD_DOCKER_IMAGE:[[:space:]]*//'
}

is_healthy() {
    docker inspect --format="{{.State.Health.Status}}" "$CONTAINER_NAME" 2>/dev/null | grep -q "healthy"
}

# True if something is already accepting connections on the rippled WS port,
# whether that's our own container, a manually-run rippled, or a native build.
# Checked with a bare TCP dial (bash's /dev/tcp) so this works even without Docker installed.
is_port_open() {
    (: < "/dev/tcp/127.0.0.1/$RIPPLED_WS_PORT") 2>/dev/null
}

start() {
    if is_healthy; then
        echo "✅ $CONTAINER_NAME is already running and healthy."
        return 0
    fi

    if is_port_open; then
        echo "✅ Something is already listening on ws://localhost:$RIPPLED_WS_PORT - assuming rippled is already running; skipping Docker."
        return 0
    fi

    require_docker

    # A stopped/unhealthy container from a previous run shouldn't block a fresh start.
    docker rm -f "$CONTAINER_NAME" &> /dev/null || true

    local image
    image="$(get_image)"
    if [[ -z "$image" ]]; then
        echo "❌ Could not read XRPLD_DOCKER_IMAGE from $WORKFLOW_FILE" >&2
        exit 1
    fi

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
    *) usage ;;
esac
