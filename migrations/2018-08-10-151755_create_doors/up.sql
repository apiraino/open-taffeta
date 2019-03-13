CREATE TABLE doors (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL UNIQUE,
    address VARCHAR NOT NULL UNIQUE,
    ring BOOLEAN NOT NULL DEFAULT 0,
    ring_ts INTEGER
);

-- example
-- INSERT INTO doors (name, address) VALUES ("front-door", "https://buzzer.mydomain.de");
