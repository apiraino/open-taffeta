CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO users (email, password) VALUES ("apiraino@users.noreply.github.com", "123456");
INSERT INTO users (email, password) VALUES ("kkom@users.noreply.github.com", "654321");
