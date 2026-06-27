#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
ADMIN_WEB_DIR="${REPO_ROOT}/runtime/admin_web"

cd "${ADMIN_WEB_DIR}"

if [[ ! -d node_modules ]]; then
  npm install
fi

npm run build
