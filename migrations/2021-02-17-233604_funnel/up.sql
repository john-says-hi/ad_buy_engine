CREATE TABLE funnel_table (
  funnel_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  funnel_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);
