CREATE TABLE traffic_source_table (
  traffic_source_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  traffic_source_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);