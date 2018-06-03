CREATE TABLE users (
       id INTEGER PRIMARY KEY not null,
       username TEXT not null,
       password TEXT not null,
       email TEXT not null
);

INSERT INTO users (username, password, email) VALUES ("apiraino", "123456", "apiraino@users.noreply.github.com");
INSERT INTO users (username, password, email) VALUES ("kkom", "654321", "kkom@users.noreply.github.com");
