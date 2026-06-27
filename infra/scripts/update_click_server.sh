#!/usr/bin/env bash
set -euo pipefail

ABE_RELEASE_REPO="${ABE_RELEASE_REPO:-john-says-hi/ad_buy_engine}"
ABE_VERSION="${ABE_VERSION:-latest}"
ABE_INSTALL_DIR="${ABE_INSTALL_DIR:-/opt/ad_buy_engine}"
ABE_BINARY_NAME="${ABE_BINARY_NAME:-click_server}"
ABE_SERVICE_NAME="${ABE_SERVICE_NAME:-ad_buy_engine_click_server.service}"

require_root() {
  if [[ "${EUID}" -ne 0 ]]; then
    echo "Run this updater as root." >&2
    exit 1
  fi
}

detect_release_asset() {
  local machine
  machine="$(uname -m)"

  case "${machine}" in
    x86_64|amd64)
      echo "click_server-${ABE_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
      ;;
    aarch64|arm64)
      echo "click_server-${ABE_VERSION}-aarch64-unknown-linux-gnu.tar.gz"
      ;;
    *)
      echo "Unsupported CPU architecture: ${machine}" >&2
      exit 1
      ;;
  esac
}

release_base_url() {
  if [[ "${ABE_VERSION}" == "latest" ]]; then
    echo "https://github.com/${ABE_RELEASE_REPO}/releases/latest/download"
  else
    echo "https://github.com/${ABE_RELEASE_REPO}/releases/download/${ABE_VERSION}"
  fi
}

download_update() {
  local asset_name release_url tmp_dir backup_path
  asset_name="$(detect_release_asset)"
  release_url="$(release_base_url)"
  tmp_dir="$(mktemp -d)"
  backup_path="${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}.previous"

  curl -fsSL "${release_url}/${asset_name}" -o "${tmp_dir}/${asset_name}"
  curl -fsSL "${release_url}/${asset_name}.sha256" -o "${tmp_dir}/${asset_name}.sha256"
  (cd "${tmp_dir}" && sha256sum -c "${asset_name}.sha256")
  tar -xzf "${tmp_dir}/${asset_name}" -C "${tmp_dir}"

  if [[ -f "${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}" ]]; then
    cp "${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}" "${backup_path}"
  fi

  install -m 0755 "${tmp_dir}/${ABE_BINARY_NAME}" "${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}"
  if [[ -d "${tmp_dir}/admin_web" ]]; then
    rm -rf "${ABE_INSTALL_DIR}/admin_web"
    cp -R "${tmp_dir}/admin_web" "${ABE_INSTALL_DIR}/admin_web"
  fi

  rm -rf "${tmp_dir}"
}

restart_service() {
  systemctl restart "${ABE_SERVICE_NAME}"
  systemctl --no-pager --full status "${ABE_SERVICE_NAME}" || true
}

main() {
  require_root
  download_update
  restart_service
  echo "Ad Buy Engine click server updated."
}

main "$@"
