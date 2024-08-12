-- Your SQL goes here
CREATE TABLE user_role (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);
INSERT INTO user_role (name) VALUES('Player'), ('MJ'), ('Administrator');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  password VARCHAR not null,
  role SMALLINT not null default 1,
  created_at TIMESTAMP not null,
  updated_at TIMESTAMP not null,
  CONSTRAINT fk_user_role FOREIGN KEY (role) REFERENCES user_role(id)
);
INSERT INTO users(email, username, password, role, created_at, updated_at) VALUES('admin@admin.com', 'Admin', '7570a74a41cc13f013d2a1f9ce81e88f', 3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);