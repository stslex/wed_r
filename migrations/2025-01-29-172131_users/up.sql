-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    uuid UUID DEFAULT uuid_generate_v4 () NOT NULL CONSTRAINT users_pk PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_uuid_uindex ON users (uuid);

CREATE INDEX IF NOT EXISTS users_username_index ON users (username);

CREATE INDEX IF NOT EXISTS users_name_index ON users (name);
