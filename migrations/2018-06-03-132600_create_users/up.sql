CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

INSERT INTO users (id, username, password, email) VALUES (0, "apiraino", "0123456", "apiraino@users.noreply.github.com");
INSERT INTO users (id, username, password, email) VALUES (1, "kkom", "6543210", "kkom@users.noreply.github.com");
