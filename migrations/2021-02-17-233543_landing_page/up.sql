CREATE TABLE landing_page_table (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    is_pre_landing_page BOOL NOT NULL,
    clearance VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    tags VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    url_tokens VARCHAR NOT NULL,
    number_of_calls_to_action VARCHAR NOT NULL,
    vertical VARCHAR NOT NULL,
    language VARCHAR NOT NULL,
    notes VARCHAR NOT NULL,
    archived BOOL NOT NULL,
    last_updated BIGINT NOT NULL
);