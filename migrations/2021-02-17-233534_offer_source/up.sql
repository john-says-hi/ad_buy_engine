CREATE TABLE offer_sources (
    id VARCHAR(48) NOT NULL PRIMARY KEY,
    account_id VARCHAR(48) NOT NULL,
    name VARCHAR NOT NULL,
    clearance VARCHAR NOT NULL,
    click_id_token VARCHAR NOT NULL,
    payout_token VARCHAR NOT NULL,
    conversion_id_token VARCHAR NOT NULL,
    custom_events VARCHAR NOT NULL,
    tracking_domain VARCHAR NOT NULL,
    conversion_tracking_method VARCHAR NOT NULL,
    include_additional_parameters_in_postback_url BOOL NOT NULL,
    payout_currency VARCHAR NOT NULL,
    append_click_id BOOL NOT NULL,
    accept_duplicate_post_backs BOOL NOT NULL,
    whitelisted_postback_ips VARCHAR NOT NULL,
    referrer_handling VARCHAR NOT NULL,
    notes VARCHAR NOT NULL,
    archived BOOL NOT NULL,
    last_updated BIGINT NOT NULL
);

