CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS admin_users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    admin_user_id TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (admin_user_id) REFERENCES admin_users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);

CREATE TABLE IF NOT EXISTS campaigns (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    destination_url TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_campaigns_slug ON campaigns(slug);
CREATE INDEX IF NOT EXISTS idx_campaigns_is_active ON campaigns(is_active);

CREATE TABLE IF NOT EXISTS click_events (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL,
    slug TEXT NOT NULL,
    destination_url TEXT NOT NULL,
    request_query TEXT,
    referrer TEXT,
    ip_address TEXT,
    user_agent TEXT,
    redirect_status_code INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_click_events_campaign_id ON click_events(campaign_id);
CREATE INDEX IF NOT EXISTS idx_click_events_created_at ON click_events(created_at);

CREATE TABLE IF NOT EXISTS domain_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    tracking_domain TEXT,
    https_enabled INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);
