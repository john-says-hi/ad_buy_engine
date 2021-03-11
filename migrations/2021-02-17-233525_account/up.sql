CREATE TABLE account_table (
  account_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);