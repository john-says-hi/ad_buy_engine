CREATE TABLE offer_source_table (
  offer_source_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  offer_source_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);

