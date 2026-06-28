# Ad Buy Engine Project Architecture

This public repository contains the open Ad Buy Engine code. Private operations,
secrets, planning artifacts, and infrastructure notes belong outside this repo in
`ad_buy_engine_workspace/`.

## Top-Level Areas

- `feats/` contains product-facing feature crates. Current code:
  `feats/admin_dashboard/`, a Yew admin dashboard that authenticates against
  the local campaign server and manages campaign entities through REST APIs.
- `core/` contains pure shared domain/API contracts. Current code:
  `core/domain/`, the serde DTOs used by the dashboard and campaign server.
- `runtime/` contains executable runtime services. Current code:
  `runtime/campaign_server/`, an Axum + SQLx SQLite server for auth, CRUD,
  funnel evaluation, visit/event persistence, static dashboard serving, and
  campaign click redirects.
- `infra/` contains local scripts and automation. Current code:
  `infra/scripts/run_frontend_dev.sh`, `infra/scripts/check_frontend.sh`, and
  `infra/scripts/ad_buy_engine_local`.
- `legacy/` is read-only reference code for the previous implementation.
  New work should not move or rewrite it unless a task explicitly says so.

## Public Runtime

`feats/admin_dashboard/` is a Trunk/Yew CSR app using Rust 1.90.0,
edition 2024, Yew 0.23.0, and Yew Router 0.20.0.

The dashboard is served by `runtime/campaign_server/` in local mode. On a fresh
SQLite database, log in with `admin` / `admin`; the app then requires an
operator credential change before CRUD pages are usable.

The server exposes:

- auth/session APIs under `/api/auth/*`
- campaign element CRUD under `/api/{offer-sources,offers,landers,traffic-sources,funnels,campaigns}`
- dropdown data under `/api/options/*`
- campaign click links at `/c/{campaign_id}`
- lander CTA continuation links at `/go/{visit_id}/{slot}`
- static dashboard fallback for non-API routes

## Local Validation

Run the local app from the public repo root:

```bash
./ad_buy_engine_local
```

The runner builds the dashboard, creates or migrates the SQLite database at
`runtime/data/ad_buy_engine.sqlite3` by default, and serves the dashboard at
`http://127.0.0.1:8088`. Override runtime settings with private env vars in the
neighboring `abe_private_ops/.env` file or with exported `ABE_*` variables.

Run quality gates from the public repo root:

```bash
infra/scripts/check_frontend.sh
cargo nextest run --workspace
```

Start local development:

```bash
infra/scripts/run_frontend_dev.sh
```
