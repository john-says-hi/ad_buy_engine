CREATE TABLE invitation_table (
  invitation_id VARCHAR(36) NOT NULL PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  email_confirmed BOOLEAN NOT NULL,
  expires_at TIMESTAMP NOT NULL
);