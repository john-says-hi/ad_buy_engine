CREATE TABLE visit_table (
  click_id VARCHAR(36) NOT NULL PRIMARY KEY,
  account_id VARCHAR(36) NOT NULL,
  visit_data VARCHAR NOT NULL,
  last_updated BIGINT NOT NULL
);
