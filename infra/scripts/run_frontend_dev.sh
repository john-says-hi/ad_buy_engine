#!/usr/bin/env bash
set -euo pipefail

TRUNK_VERSION="0.21.14"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
APP_DIR="${REPO_ROOT}/feats/admin_dashboard"
PORT="${ABE_FRONTEND_PORT:-8080}"

ensure_wasm_target() {
  rustup target add wasm32-unknown-unknown
}

ensure_trunk() {
  if command -v trunk >/dev/null 2>&1 && env -u NO_COLOR trunk --version | grep -q "trunk ${TRUNK_VERSION}"; then
    return
  fi

  echo "Installing trunk ${TRUNK_VERSION} with cargo install."
  cargo install trunk --version "${TRUNK_VERSION}" --locked
}

ensure_wasm_target
ensure_trunk

cd "${APP_DIR}"
exec env -u NO_COLOR trunk serve --address 127.0.0.1 --port "${PORT}"
