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
  role_id SERIAL not null,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_user_role FOREIGN KEY (role_id) REFERENCES user_role(id)
);
INSERT INTO users(email, username, password, role_id) VALUES('admin@admin.com', 'Admin', '7570a74a41cc13f013d2a1f9ce81e88f', 3);