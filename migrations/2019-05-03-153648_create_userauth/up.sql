CREATE TABLE userauth (
       id INTEGER PRIMARY KEY NOT NULL,
       user_id INTEGER NOT NULL,
       exp DATETIME NOT NULL,
       client_id VARCHAR NOT NULL,
       token VARCHAR NOT NULL UNIQUE
);
