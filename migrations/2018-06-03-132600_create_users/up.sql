CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO users (username, password, email) VALUES ("apiraino", "123456", "apiraino@users.noreply.github.com");
INSERT INTO users (username, password, email) VALUES ("kkom", "654321", "kkom@users.noreply.github.com");
