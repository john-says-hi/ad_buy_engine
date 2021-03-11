CREATE TABLE offer_table (
  offer_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  offer_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);