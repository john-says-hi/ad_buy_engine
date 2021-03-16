CREATE TABLE traffic_source_table (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    name VARCHAR NOT NULL,
    clearance VARCHAR NOT NULL,
    external_id_token_data VARCHAR NOT NULL,
    cost_token_data VARCHAR NOT NULL,
    custom_token_data VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    traffic_source_postback_url VARCHAR NOT NULL,
    traffic_source_postback_url_on_custom_event VARCHAR NOT NULL,
    pixel_redirect_url VARCHAR NOT NULL,
    track_impressions BOOL NOT NULL,
    direct_tracking BOOL NOT NULL,
    notes VARCHAR NOT NULL,
    archived BOOL NOT NULL,
    last_updated BIGINT NOT NULL
);