# Security Policy

## Current Status

Ad Buy Engine is a legacy prototype being prepared for public redevelopment. It
is not currently a production-ready application.

## Secrets

Do not commit real credentials, API keys, private keys, database URLs, or local
`.env` files. Use `.env.example` for placeholders only.

If you accidentally expose a secret:

1. Revoke or rotate it immediately.
2. Remove it from the current tree.
3. Rewrite repository history if the value was committed.
4. Re-scan the repository before making it public again.

## Known Legacy Risk

The current Rust dependency tree includes advisories from old Actix/Diesel-era
dependencies. Do not deploy this code as-is. Dependency modernization should be
handled as a separate refactor before any real production use.
