CREATE TABLE users (
  id VARCHAR(48) NOT NULL PRIMARY KEY,
  account_id VARCHAR(48) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  last_updated BIGINT NOT NULL
);

--insert into users (id, first_name, last_name, email, password, created_by, updated_by) values
--('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000',  'admin', 'user', 'admin@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
--('1802d2f8-1a18-43c1-9c58-1c3f7100c842', '1802d2f8-1a18-43c1-9c58-1c3f7100c862', 'test', 'user', 'test@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000');