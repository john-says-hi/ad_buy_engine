# Ad Buy Engine Project Architecture

This public repository contains the open Ad Buy Engine code. Private operations,
secrets, planning artifacts, and infrastructure notes belong outside this repo in
`ad_buy_engine_workspace/`.

## Top-Level Areas

- `feats/` contains product-facing feature crates. Current code:
  `feats/admin_dashboard/`, a static Yew admin dashboard prototype.
- `infra/` contains local scripts and automation. Current code:
  `infra/scripts/run_frontend_dev.sh` and `infra/scripts/check_frontend.sh`.
- `legacy/` is read-only reference code for the previous implementation.
  New work should not move or rewrite it unless a task explicitly says so.

## Frontend Prototype

`feats/admin_dashboard/` is a Trunk/Yew CSR app using Rust 1.90.0,
edition 2024, Yew 0.23.0, and Yew Router 0.20.0.

The prototype is appearance-only:

- no login
- no backend API calls
- no click-server behavior
- no real campaign data

Legacy visual assets are copied into `public/assets/` for local, network-free
rendering. UI state is static and testable through route, report, and create
form metadata. Creatable report pages open static legacy-style modal forms; save
actions remain intentionally disconnected until backend work is added.

## Local Validation

Run the frontend quality gates from the public repo root:

```bash
infra/scripts/check_frontend.sh
```

Start local development:

```bash
infra/scripts/run_frontend_dev.sh
```
