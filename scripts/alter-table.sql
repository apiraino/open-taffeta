BEGIN TRANSACTION;
CREATE TEMPORARY TABLE users_backup (
       id INTEGER PRIMARY KEY NOT NULL,
       password VARCHAR NOT NULL,
       email VARCHAR NOT NULL UNIQUE,
       is_active BOOLEAN NOT NULL DEFAULT 0);
INSERT INTO users_backup SELECT id,email,password,is_active FROM users;
DROP TABLE users;
CREATE TABLE users (
       id INTEGER PRIMARY KEY NOT NULL,
       password VARCHAR NOT NULL,
       email VARCHAR NOT NULL UNIQUE,
       is_active BOOLEAN NOT NULL DEFAULT 0);
INSERT INTO users SELECT id,email,password,is_active FROM users_backup;
DROP TABLE users_backup;
COMMIT;
