CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO users (username, password, email) VALUES ("apiraino", "0123456", "apiraino@users.noreply.github.com");
INSERT INTO users (username, password, email) VALUES ("kkom", "6543210", "kkom@users.noreply.github.com");
