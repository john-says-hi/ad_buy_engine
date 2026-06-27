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
configuration in the sibling `../abe_private_ops` directory instead of this public
repository.

```text
ad_buy_engine_workspace/
  ad_buy_engine/   # public GitHub repository
  abe_private_ops/ # private GitHub repository for operating files
```

For Docker Compose from this workspace layout, pass the private env file
explicitly:

```shell
AD_BUY_ENGINE_ENV_FILE=../abe_private_ops/.env docker compose --env-file ../abe_private_ops/.env config --quiet
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
