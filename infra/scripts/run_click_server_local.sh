#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
LOCAL_DIR="${REPO_ROOT}/runtime/click_server/.local"

mkdir -p "${LOCAL_DIR}"

export ABE_BIND_ADDR="${ABE_BIND_ADDR:-127.0.0.1:8080}"
export ABE_DATABASE_URL="${ABE_DATABASE_URL:-sqlite://${LOCAL_DIR}/click_server.sqlite3}"
export ABE_SETUP_SECRET="${ABE_SETUP_SECRET:-local-dev-setup-secret}"
export ABE_ADMIN_DIST_DIR="${ABE_ADMIN_DIST_DIR:-${REPO_ROOT}/runtime/admin_web/dist}"
export ABE_COOKIE_SECURE="${ABE_COOKIE_SECURE:-false}"
export RUST_LOG="${RUST_LOG:-click_server=info,tower_http=info}"

echo "Ad Buy Engine click server"
echo "URL: http://${ABE_BIND_ADDR}"
echo "Setup secret: ${ABE_SETUP_SECRET}"

cd "${REPO_ROOT}"
cargo run -p click_server
