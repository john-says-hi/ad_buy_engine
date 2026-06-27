# Click Server Prototype

The first modern Add Buy Engine slice is a small self-hosted click server.
It is built to run locally or on a small Ubuntu VPS with SQLite.

## Local Run

```shell
infra/scripts/build_admin_web.sh
infra/scripts/run_click_server_local.sh
```

Open `http://127.0.0.1:8080`.
The default local setup secret is `local-dev-setup-secret` unless
`ABE_SETUP_SECRET` is set.

## VPS Install

The installer expects a GitHub Release containing:

```text
click_server-<version>-x86_64-unknown-linux-gnu.tar.gz
click_server-<version>-x86_64-unknown-linux-gnu.tar.gz.sha256
click_server-<version>-aarch64-unknown-linux-gnu.tar.gz
click_server-<version>-aarch64-unknown-linux-gnu.tar.gz.sha256
```

Create the current machine artifact:

```shell
infra/scripts/package_click_server_release.sh
```

Run the installer on a VPS:

```shell
curl -fsSL https://raw.githubusercontent.com/john-says-hi/ad_buy_engine/master/infra/scripts/install_click_server.sh | sudo bash
```

The installer creates:

```text
/etc/ad_buy_engine/click_server.env
/var/lib/ad_buy_engine/click_server.sqlite3
/var/log/ad_buy_engine/
/opt/ad_buy_engine/
```

It prints the setup URL and one-time setup secret when complete.

## Prototype API

- `GET /health`
- `GET /api/version`
- `GET /api/setup/status`
- `POST /api/setup/complete`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `GET /api/session`
- `GET /api/campaigns`
- `POST /api/campaigns`
- `PUT /api/campaigns/:id`
- `DELETE /api/campaigns/:id`
- `GET /c/:slug`
- `GET /api/stats/summary`
- `GET /api/stats/campaign/:id`
- `GET /api/maintenance/status`

## Deferred

ClickHouse, sync workers, ML, multi-user permissions, and database switching are
intentionally deferred until the SQLite prototype is useful.
