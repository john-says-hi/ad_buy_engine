CREATE TABLE campaigns (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    clearance VARCHAR NOT NULL,
    traffic_source VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    cost_model VARCHAR NOT NULL,
    cost_value VARCHAR NOT NULL,
    redirect_option VARCHAR NOT NULL,
    campaign_destination VARCHAR NOT NULL,
    campaign_core VARCHAR NOT NULL,
    notes VARCHAR NOT NULL,
    archived BOOL NOT NULL,
    last_updated BIGINT NOT NULL,
    last_clicked BIGINT NOT NULL,
    hosts  VARCHAR NOT NULL
);