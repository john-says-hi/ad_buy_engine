CREATE TABLE campaign_table (
  campaign_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  campaign_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);