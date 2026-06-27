#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
VERSION="${ABE_VERSION:-$(grep '^version = ' "${REPO_ROOT}/runtime/click_server/Cargo.toml" | head -n1 | cut -d '"' -f2)}"
TARGET_TRIPLE="${TARGET_TRIPLE:-$(rustc -vV | awk '/host:/ {print $2}')}"
OUT_DIR="${REPO_ROOT}/target/release-artifacts"
ASSET_NAME="click_server-${VERSION}-${TARGET_TRIPLE}"
STAGING_DIR="${OUT_DIR}/${ASSET_NAME}"

rm -rf "${STAGING_DIR}"
mkdir -p "${STAGING_DIR}" "${OUT_DIR}"

"${REPO_ROOT}/infra/scripts/build_admin_web.sh"

cd "${REPO_ROOT}"
cargo build -p click_server --release --target "${TARGET_TRIPLE}"

install -m 0755 \
  "${REPO_ROOT}/target/${TARGET_TRIPLE}/release/click_server" \
  "${STAGING_DIR}/click_server"
cp -R "${REPO_ROOT}/runtime/admin_web/dist" "${STAGING_DIR}/admin_web"

tar -C "${OUT_DIR}" -czf "${OUT_DIR}/${ASSET_NAME}.tar.gz" "${ASSET_NAME}"
(cd "${OUT_DIR}" && sha256sum "${ASSET_NAME}.tar.gz" > "${ASSET_NAME}.tar.gz.sha256")

echo "Created ${OUT_DIR}/${ASSET_NAME}.tar.gz"
echo "Created ${OUT_DIR}/${ASSET_NAME}.tar.gz.sha256"
