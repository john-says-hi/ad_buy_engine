# Ad Buy Engine

## Project status

This repository is a legacy prototype. It is being made public for reference and
future redevelopment, but the current server stack is not production-ready.

Before deploying anything from this repo:

- Create a local `.env` from `.env.example`; never commit real secrets.
- Rotate any old credentials that may have existed before the repository
  history cleanup.
- Review `cargo audit` output and upgrade the legacy Actix/Diesel-era
  dependencies.
- Treat the current deployment scripts as historical helpers, not a supported
  production release path.

## Local configuration

Copy `.env.example` to `.env` and replace the placeholder values before running
the server locally. The `.env` file is intentionally ignored by git.

If you are using the recommended local workspace layout, keep real private
configuration in the private workspace root instead of this public repository.

```text
ad_buy_engine_workspace/
  .env           # private runtime config
  ad_buy_engine/ # public GitHub repository
```

For Docker Compose from this workspace layout, pass the private env file
explicitly:

```shell
AD_BUY_ENGINE_ENV_FILE=../.env docker compose --env-file ../.env config --quiet
```

## Docker

```shell
docker compose config --quiet
docker compose up --build
```

## Checks

```shell
cargo check -p ad_buy_engine
cargo audit
```

## New click server prototype

The new prototype lives beside the legacy code:

```text
runtime/click_server/ # Axum + SQLite tracking server
runtime/admin_web/    # React + Vite admin UI
infra/scripts/        # local, install, update, and release helpers
```

Build the admin UI:

```shell
infra/scripts/build_admin_web.sh
```

Run the local click server:

```shell
infra/scripts/run_click_server_local.sh
```

Then open `http://127.0.0.1:8080` and use the printed setup secret.

Focused server checks:

```shell
cargo check -p click_server
cargo nextest run -p click_server
```

If `cargo nextest` is not installed yet:

```shell
cargo install cargo-nextest --locked
```

Create release artifacts for a GitHub Release:

```shell
infra/scripts/package_click_server_release.sh
```

Install on a small Ubuntu VPS from a release:

```shell
curl -fsSL https://raw.githubusercontent.com/john-says-hi/ad_buy_engine/master/infra/scripts/install_click_server.sh | sudo bash
```
