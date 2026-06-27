#!/usr/bin/env bash
set -euo pipefail

ABE_RELEASE_REPO="${ABE_RELEASE_REPO:-john-says-hi/ad_buy_engine}"
ABE_VERSION="${ABE_VERSION:-latest}"
ABE_BIND_ADDR="${ABE_BIND_ADDR:-0.0.0.0:8080}"
ABE_SERVICE_USER="${ABE_SERVICE_USER:-adbuyengine}"
ABE_INSTALL_DIR="${ABE_INSTALL_DIR:-/opt/ad_buy_engine}"
ABE_CONFIG_DIR="${ABE_CONFIG_DIR:-/etc/ad_buy_engine}"
ABE_DATA_DIR="${ABE_DATA_DIR:-/var/lib/ad_buy_engine}"
ABE_LOG_DIR="${ABE_LOG_DIR:-/var/log/ad_buy_engine}"
ABE_SETUP_SECRET="${ABE_SETUP_SECRET:-}"
ABE_BINARY_NAME="${ABE_BINARY_NAME:-click_server}"

require_root() {
  if [[ "${EUID}" -ne 0 ]]; then
    echo "Run this installer as root." >&2
    exit 1
  fi
}

install_packages() {
  apt-get update
  DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates curl openssl tar ufw
}

ensure_runtime_user() {
  if ! id "${ABE_SERVICE_USER}" >/dev/null 2>&1; then
    useradd --system --home "${ABE_DATA_DIR}" --shell /usr/sbin/nologin "${ABE_SERVICE_USER}"
  fi
}

ensure_directories() {
  install -d -m 0755 "${ABE_INSTALL_DIR}" "${ABE_CONFIG_DIR}" "${ABE_LOG_DIR}"
  install -d -m 0750 -o "${ABE_SERVICE_USER}" -g "${ABE_SERVICE_USER}" "${ABE_DATA_DIR}"
}

generate_setup_secret() {
  if [[ -z "${ABE_SETUP_SECRET}" ]]; then
    ABE_SETUP_SECRET="$(openssl rand -hex 32)"
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

download_release() {
  local asset_name release_url tmp_dir
  asset_name="$(detect_release_asset)"
  release_url="$(release_base_url)"
  tmp_dir="$(mktemp -d)"

  curl -fsSL "${release_url}/${asset_name}" -o "${tmp_dir}/${asset_name}"
  curl -fsSL "${release_url}/${asset_name}.sha256" -o "${tmp_dir}/${asset_name}.sha256"
  (cd "${tmp_dir}" && sha256sum -c "${asset_name}.sha256")
  tar -xzf "${tmp_dir}/${asset_name}" -C "${tmp_dir}"

  install -m 0755 "${tmp_dir}/${ABE_BINARY_NAME}" "${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}"
  if [[ -d "${tmp_dir}/admin_web" ]]; then
    rm -rf "${ABE_INSTALL_DIR}/admin_web"
    cp -R "${tmp_dir}/admin_web" "${ABE_INSTALL_DIR}/admin_web"
  fi

  rm -rf "${tmp_dir}"
}

write_environment_file() {
  cat > "${ABE_CONFIG_DIR}/click_server.env" <<EOF
ABE_BIND_ADDR=${ABE_BIND_ADDR}
ABE_DATABASE_URL=sqlite://${ABE_DATA_DIR}/click_server.sqlite3
ABE_SETUP_SECRET=${ABE_SETUP_SECRET}
ABE_ADMIN_DIST_DIR=${ABE_INSTALL_DIR}/admin_web
ABE_COOKIE_SECURE=false
ABE_SESSION_TTL_SECONDS=604800
RUST_LOG=click_server=info,tower_http=info
EOF

  chmod 0600 "${ABE_CONFIG_DIR}/click_server.env"
  chown root:"${ABE_SERVICE_USER}" "${ABE_CONFIG_DIR}/click_server.env"
}

write_systemd_unit() {
  cat > /etc/systemd/system/ad_buy_engine_click_server.service <<EOF
[Unit]
Description=Ad Buy Engine Click Server
After=network-online.target
Wants=network-online.target

[Service]
User=${ABE_SERVICE_USER}
Group=${ABE_SERVICE_USER}
EnvironmentFile=${ABE_CONFIG_DIR}/click_server.env
ExecStart=${ABE_INSTALL_DIR}/${ABE_BINARY_NAME}
Restart=always
RestartSec=5
WorkingDirectory=${ABE_DATA_DIR}
ReadWritePaths=${ABE_DATA_DIR} ${ABE_LOG_DIR}
NoNewPrivileges=true

[Install]
WantedBy=multi-user.target
EOF

  systemctl daemon-reload
  systemctl enable ad_buy_engine_click_server.service
}

configure_firewall() {
  ufw allow 22/tcp >/dev/null || true
  ufw allow 80/tcp >/dev/null || true
  ufw allow 443/tcp >/dev/null || true
  ufw allow 8080/tcp >/dev/null || true
}

start_service() {
  systemctl restart ad_buy_engine_click_server.service
}

print_summary() {
  local public_ip
  public_ip="$(curl -fsSL https://api.ipify.org || echo "<server-ip>")"

  echo
  echo "Ad Buy Engine click server installed."
  echo "Setup URL: http://${public_ip}:8080"
  echo "Setup secret: ${ABE_SETUP_SECRET}"
  echo "Service: ad_buy_engine_click_server.service"
}

main() {
  require_root
  install_packages
  ensure_runtime_user
  ensure_directories
  generate_setup_secret
  download_release
  write_environment_file
  write_systemd_unit
  configure_firewall
  start_service
  print_summary
}

main "$@"
