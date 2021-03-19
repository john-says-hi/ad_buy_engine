CREATE TABLE funnels (
    id VARCHAR(48) NOT NULL PRIMARY KEY,
    account_id VARCHAR(48) NOT NULL,
    country VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    clearance VARCHAR NOT NULL,
    redirect_option VARCHAR NOT NULL,
    referrer_handling VARCHAR NOT NULL,
    notes VARCHAR NOT NULL,
    conditional_sequences VARCHAR NOT NULL,
    default_sequences VARCHAR NOT NULL,
    archived BOOL NOT NULL,
    last_updated BIGINT NOT NULL
);
