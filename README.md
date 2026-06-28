# Ad Buy Engine

## Run and Test Local

From the private workspace root, run:

```bash
./ad_buy_engine/ad_buy_engine_local
```

The local runner loads private environment values from `../abe_private_ops/.env` when present. URL roles can be set explicitly:

```bash
ABE_TRACKING_BASE_URL=http://127.0.0.1:8088
ABE_ADMIN_BASE_URL=http://127.0.0.1:8088
```

When those role variables are not set, `ABE_PUBLIC_BASE_URL` is still accepted as a temporary fallback for both roles.

Campaign links are generated as:

```text
{ABE_TRACKING_BASE_URL}/c/{campaign_id}
```

The dashboard uses `ABE_ADMIN_BASE_URL` for display/access settings.

## VPS Install

The first server installer targets Ubuntu 24.04 LTS only. Before running it:

- Create DNS `A` and/or `AAAA` records for the primary domain.
- Point those records at the VPS public IP.
- Make sure ports `80` and `443` can reach the server.

On the VPS:

```bash
curl -fsSL https://raw.githubusercontent.com/john-says-hi/ad_buy_engine/main/infra/scripts/install_on_server -o /tmp/install_ad_buy_engine
chmod +x /tmp/install_ad_buy_engine
sudo /tmp/install_ad_buy_engine --domain track.example.com
```

Optional:

```bash
sudo /tmp/install_ad_buy_engine --domain track.example.com --git-ref main
```

Preview planned actions without changing the server:

```bash
infra/scripts/install_on_server --dry-run
```

The installer is safe to rerun. It updates the repo checkout, rebuilds the dashboard and release server binary, rewrites managed config, restarts the service, and replaces only the managed Ad Buy Engine block in the Caddyfile.

## Server Paths

- Runtime env: `/etc/ad_buy_engine/ad_buy_engine.env`
- SQLite database: `/var/lib/ad_buy_engine/ad_buy_engine.sqlite3`
- Repo checkout: `/opt/ad_buy_engine/repo`
- Server binary: `/opt/ad_buy_engine/bin/campaign_server`
- Systemd service: `ad_buy_engine.service`
- Logs: `journalctl -u ad_buy_engine -f`
- HTTPS proxy: Caddy, reverse proxying to `127.0.0.1:8088`

The app backend stays bound to localhost. UFW opens only SSH, HTTP, and HTTPS.

## Verify Deployment

After install:

```bash
systemctl status ad_buy_engine
curl -I https://track.example.com
```

Log in with the default `admin/admin` credentials, then change them when prompted. In Domain Settings, save the primary domain. Create or edit a campaign and confirm its tracking URL uses:

```text
https://track.example.com/c/{campaign_id}
```

## Manual VPS Updates

Tagged GitHub releases can produce a public release package with:

- `campaign_server`
- dashboard `dist/`
- `manifest.json`
- package `.sha256`

The release manifest records the version, git SHA, target triple, schema compatibility, rollback policy, and SHA-256 values for package files. The VPS update flow is manual: an authenticated operator opens Settings → Updates, checks the latest release, types the confirmation text, and starts install or rollback.

The web app only writes update requests into `ABE_UPDATE_CONTROL_DIR`. The separate `update_agent` service performs the privileged work:

1. Download GitHub release asset.
2. Verify manifest and file digests.
3. Stage the release into the inactive slot.
4. Start the inactive systemd slot.
5. Check local slot health.
6. Switch the Nginx upstream.
7. Reload Nginx after `nginx -t`.
8. Check public health.
9. Drain and stop the old slot.

Default slots:

```text
blue  = 127.0.0.1:18081
green = 127.0.0.1:18082
```

Public-safe templates live under `infra/systemd/` and `infra/nginx/`. Real VPS values stay in the private `abe_private_ops/.env`, including:

```text
ABE_RELEASE_ROOT=/opt/ad_buy_engine/releases
ABE_UPDATE_CONTROL_DIR=/var/lib/ad_buy_engine/update_control
ABE_NGINX_ACTIVE_UPSTREAM=/etc/nginx/conf.d/ad-buy-engine-upstream.conf
ABE_PUBLIC_HEALTH_URL=https://track.example.com/api/health
ABE_DATABASE_URL=sqlite:/var/lib/ad_buy_engine/ad_buy_engine.sqlite3
ABE_PUBLIC_BASE_URL=https://track.example.com
ABE_CURRENT_SCHEMA_VERSION=3
```

Install or dry-run the template setup from the public repo checkout:

```bash
infra/scripts/install_vps_update_system --dry-run --env-file ../abe_private_ops/.env
sudo infra/scripts/install_vps_update_system --env-file ../abe_private_ops/.env
```

No update starts automatically. Rollback is offered only when the installed release manifest marks schema rollback as safe.
