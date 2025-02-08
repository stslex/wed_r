-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    uuid UUID DEFAULT uuid_generate_v4 () NOT NULL CONSTRAINT users_pk PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    name TEXT NOT NULL,
    chat_id BIGINT,
    is_accepted BOOLEAN NOT NULL
);

ALTER TABLE users
ADD COLUMN IF NOT EXISTS chat_id BIGINT;

ALTER TABLE users
DROP COLUMN IF EXISTS is_active;

DROP INDEX IF EXISTS users_is_active_index;

CREATE UNIQUE INDEX IF NOT EXISTS users_uuid_uindex ON users (uuid);

CREATE INDEX IF NOT EXISTS users_username_index ON users (username);

CREATE INDEX IF NOT EXISTS users_name_index ON users (name);

CREATE INDEX IF NOT EXISTS users_chat_id_index ON users (chat_id);

CREATE INDEX IF NOT EXISTS users_is_accepted_index ON users (is_accepted);
