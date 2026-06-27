#!/usr/bin/env bash

# <UDF name="abe_release_repo" Label="GitHub release repo" default="john-says-hi/ad_buy_engine" />
# <UDF name="abe_version" Label="Release version" default="latest" />
# <UDF name="abe_bind_addr" Label="Bind address" default="0.0.0.0:8080" />
# <UDF name="abe_setup_secret" Label="Setup secret" default="" />

set -euo pipefail

export ABE_RELEASE_REPO="${ABE_RELEASE_REPO:-${abe_release_repo:-john-says-hi/ad_buy_engine}}"
export ABE_VERSION="${ABE_VERSION:-${abe_version:-latest}}"
export ABE_BIND_ADDR="${ABE_BIND_ADDR:-${abe_bind_addr:-0.0.0.0:8080}}"
export ABE_SETUP_SECRET="${ABE_SETUP_SECRET:-${abe_setup_secret:-}}"

curl -fsSL \
  "https://raw.githubusercontent.com/${ABE_RELEASE_REPO}/master/infra/scripts/install_click_server.sh" \
  -o /tmp/install_click_server.sh

bash /tmp/install_click_server.sh
