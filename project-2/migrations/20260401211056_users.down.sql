-- Add down migration script here: UNDO MIGRATIONS
DROP TABLE IF EXISTS "users";

DROP TYPE IF EXISTS user_role;

DROP EXTENSION IF EXISTS "uuis-ossp";
