#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
APP_DIR="${REPO_ROOT}/feats/admin_dashboard"

cd "${REPO_ROOT}"

rustup target add wasm32-unknown-unknown
cargo fmt --check
cargo check -p campaign_server
cargo nextest run --workspace
cargo check -p admin_dashboard --target wasm32-unknown-unknown

if ! command -v trunk >/dev/null 2>&1; then
  echo "trunk is missing. Run infra/scripts/run_frontend_dev.sh to install trunk 0.21.14."
  exit 1
fi

if ! env -u NO_COLOR trunk --version | grep -q "trunk 0.21.14"; then
  echo "trunk 0.21.14 is required. Run infra/scripts/run_frontend_dev.sh to install it."
  exit 1
fi

cd "${APP_DIR}"
env -u NO_COLOR trunk build
