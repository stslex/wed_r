-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS users_uuid_uindex;

DROP INDEX IF EXISTS users_username_index;

DROP INDEX IF EXISTS users_name_index;

DROP TABLE IF EXISTS users;

DROP EXTENSION IF EXISTS "uuid-ossp";
