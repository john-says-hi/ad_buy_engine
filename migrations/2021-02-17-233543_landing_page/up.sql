CREATE TABLE landing_page_table (
  landing_page_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  landing_page_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);