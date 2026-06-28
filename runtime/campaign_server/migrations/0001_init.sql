CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS operator_credentials (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    must_change_credentials INTEGER NOT NULL,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS app_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    public_base_url TEXT NOT NULL,
    primary_tracking_domain TEXT NOT NULL DEFAULT '',
    tracking_base_url TEXT NOT NULL DEFAULT '',
    admin_dashboard_domain TEXT NOT NULL DEFAULT '',
    admin_dashboard_base_url TEXT NOT NULL DEFAULT '',
    domain_setup_status TEXT NOT NULL DEFAULT 'not_configured',
    session_key_generated_at_millis INTEGER NOT NULL,
    schema_version INTEGER NOT NULL,
    app_version TEXT NOT NULL,
    maxmind_account_id TEXT NOT NULL DEFAULT '',
    maxmind_license_key TEXT NOT NULL DEFAULT '',
    geolite_city_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-City.mmdb',
    geolite_country_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-Country.mmdb',
    geolite_asn_database_path TEXT NOT NULL DEFAULT 'runtime/data/GeoLite2-ASN.mmdb',
    geolite_last_download_at_millis INTEGER,
    geolite_last_download_error TEXT,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS offer_sources (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    tokens_json TEXT NOT NULL,
    tracking_domain TEXT NOT NULL,
    tracking_method TEXT NOT NULL,
    payout_currency TEXT NOT NULL,
    postback_url TEXT NOT NULL,
    append_click_id INTEGER NOT NULL,
    accept_duplicate_postbacks INTEGER NOT NULL,
    whitelist_postback_ips_json TEXT NOT NULL,
    referrer_handling TEXT NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS offers (
    id TEXT PRIMARY KEY,
    offer_source_id TEXT NOT NULL,
    country TEXT NOT NULL,
    name TEXT NOT NULL,
    tags_json TEXT NOT NULL,
    url TEXT NOT NULL,
    url_tokens_json TEXT NOT NULL,
    payout_model TEXT NOT NULL,
    payout_value REAL NOT NULL,
    currency TEXT NOT NULL,
    language TEXT NOT NULL,
    vertical TEXT NOT NULL,
    weight INTEGER NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL,
    FOREIGN KEY (offer_source_id) REFERENCES offer_sources(id)
);

CREATE TABLE IF NOT EXISTS landing_pages (
    id TEXT PRIMARY KEY,
    country TEXT NOT NULL,
    name TEXT NOT NULL,
    tags_json TEXT NOT NULL,
    url TEXT NOT NULL,
    url_tokens_json TEXT NOT NULL,
    cta_count INTEGER NOT NULL,
    role TEXT NOT NULL DEFAULT 'standard',
    expected_conversion_event_type_ids_json TEXT NOT NULL DEFAULT '[]',
    language TEXT NOT NULL,
    vertical TEXT NOT NULL,
    weight INTEGER NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS traffic_sources (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    external_id_parameter TEXT NOT NULL,
    cost_parameter TEXT NOT NULL,
    custom_parameters_json TEXT NOT NULL,
    currency TEXT NOT NULL,
    postback_urls_json TEXT NOT NULL,
    pixel_url TEXT NOT NULL,
    track_impressions INTEGER NOT NULL,
    direct_tracking INTEGER NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS conversion_event_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    event_key TEXT NOT NULL UNIQUE,
    aliases_json TEXT NOT NULL,
    event_category TEXT NOT NULL,
    include_in_conversions INTEGER NOT NULL,
    include_in_revenue INTEGER NOT NULL,
    include_in_cost INTEGER NOT NULL,
    send_postback_to_traffic_source INTEGER NOT NULL,
    default_revenue_value REAL NOT NULL,
    currency TEXT NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS funnels (
    id TEXT PRIMARY KEY,
    country TEXT NOT NULL,
    name TEXT NOT NULL,
    redirect_handling TEXT NOT NULL,
    referrer_handling TEXT NOT NULL,
    conditional_sequences_json TEXT NOT NULL,
    default_sequences_json TEXT NOT NULL,
    notes TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS campaigns (
    id TEXT PRIMARY KEY,
    traffic_source_id TEXT NOT NULL,
    destination_type TEXT NOT NULL,
    funnel_id TEXT,
    direct_sequence_json TEXT,
    cost_model TEXT NOT NULL,
    cost_value REAL NOT NULL,
    country TEXT NOT NULL,
    name TEXT NOT NULL,
    notes TEXT NOT NULL,
    tracking_url TEXT NOT NULL,
    traffic_source_query_template TEXT NOT NULL,
    last_clicked_at_millis INTEGER,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    updated_at_millis INTEGER NOT NULL,
    FOREIGN KEY (traffic_source_id) REFERENCES traffic_sources(id),
    FOREIGN KEY (funnel_id) REFERENCES funnels(id)
);

CREATE TABLE IF NOT EXISTS visits (
    id TEXT PRIMARY KEY,
    campaign_id TEXT NOT NULL,
    traffic_source_id TEXT NOT NULL,
    selected_funnel_id TEXT,
    selected_sequence_json TEXT,
    selected_landing_page_id TEXT,
    selected_offer_id TEXT,
    referrer TEXT,
    ip_address TEXT,
    user_agent TEXT,
    country TEXT,
    region TEXT,
    city TEXT,
    timezone TEXT,
    postal_code TEXT,
    metro_code TEXT,
    asn TEXT,
    asn_organization TEXT,
    isp TEXT,
    connection_type TEXT,
    proxy_type TEXT,
    carrier TEXT,
    browser TEXT,
    browser_version TEXT,
    operating_system TEXT,
    operating_system_version TEXT,
    device_type TEXT,
    device_brand TEXT,
    device_model TEXT,
    query_params_json TEXT NOT NULL,
    click_map_json TEXT NOT NULL,
    redirect_target TEXT NOT NULL,
    suspicious INTEGER NOT NULL DEFAULT 0,
    created_at_millis INTEGER NOT NULL,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
    FOREIGN KEY (traffic_source_id) REFERENCES traffic_sources(id),
    FOREIGN KEY (selected_funnel_id) REFERENCES funnels(id),
    FOREIGN KEY (selected_landing_page_id) REFERENCES landing_pages(id),
    FOREIGN KEY (selected_offer_id) REFERENCES offers(id)
);

CREATE TABLE IF NOT EXISTS visit_events (
    id TEXT PRIMARY KEY,
    visit_id TEXT,
    campaign_id TEXT,
    event_type TEXT NOT NULL,
    event_data_json TEXT NOT NULL,
    created_at_millis INTEGER NOT NULL,
    FOREIGN KEY (visit_id) REFERENCES visits(id),
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
);

CREATE TABLE IF NOT EXISTS conversion_events (
    id TEXT PRIMARY KEY,
    visit_id TEXT,
    campaign_id TEXT NOT NULL,
    event_type_id TEXT NOT NULL,
    event_key TEXT NOT NULL,
    event_name TEXT NOT NULL,
    event_category TEXT NOT NULL,
    status TEXT NOT NULL,
    raw_status TEXT,
    revenue_value REAL NOT NULL,
    currency TEXT NOT NULL,
    transaction_id TEXT,
    external_event_id TEXT,
    identity_hash TEXT,
    dedupe_key TEXT NOT NULL,
    source TEXT NOT NULL,
    duplicate INTEGER NOT NULL,
    raw_payload_json TEXT NOT NULL,
    created_at_millis INTEGER NOT NULL,
    FOREIGN KEY (visit_id) REFERENCES visits(id),
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id),
    FOREIGN KEY (event_type_id) REFERENCES conversion_event_types(id)
);

CREATE INDEX IF NOT EXISTS idx_offer_sources_archived_name
    ON offer_sources (archived, name);
CREATE INDEX IF NOT EXISTS idx_offers_source_archived_name
    ON offers (offer_source_id, archived, name);
CREATE INDEX IF NOT EXISTS idx_landing_pages_archived_name
    ON landing_pages (archived, name);
CREATE INDEX IF NOT EXISTS idx_conversion_event_types_archived_name
    ON conversion_event_types (archived, name);
CREATE INDEX IF NOT EXISTS idx_traffic_sources_archived_name
    ON traffic_sources (archived, name);
CREATE INDEX IF NOT EXISTS idx_funnels_archived_name
    ON funnels (archived, name);
CREATE INDEX IF NOT EXISTS idx_campaigns_archived_name
    ON campaigns (archived, name);
CREATE INDEX IF NOT EXISTS idx_campaigns_last_clicked
    ON campaigns (last_clicked_at_millis);
CREATE INDEX IF NOT EXISTS idx_visits_campaign_created
    ON visits (campaign_id, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_visits_created
    ON visits (created_at_millis);
CREATE INDEX IF NOT EXISTS idx_visits_ip_user_agent
    ON visits (campaign_id, ip_address, user_agent);
CREATE INDEX IF NOT EXISTS idx_visit_events_visit_created
    ON visit_events (visit_id, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_visit_events_campaign_type
    ON visit_events (campaign_id, event_type, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_visit_events_type_created
    ON visit_events (event_type, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_visit_events_created
    ON visit_events (created_at_millis);
CREATE INDEX IF NOT EXISTS idx_conversion_events_campaign_created
    ON conversion_events (campaign_id, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_conversion_events_type_created
    ON conversion_events (event_type_id, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_conversion_events_duplicate_created
    ON conversion_events (duplicate, created_at_millis);
CREATE INDEX IF NOT EXISTS idx_conversion_events_dedupe
    ON conversion_events (dedupe_key, duplicate);
