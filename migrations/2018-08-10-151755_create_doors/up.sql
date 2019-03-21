CREATE TABLE doors (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL UNIQUE,
    address VARCHAR NOT NULL UNIQUE,
    buzzer_url VARCHAR NOT NULL UNIQUE,
    ring BOOLEAN NOT NULL DEFAULT 0,
    ring_ts INTEGER
);

-- example
-- INSERT INTO doors (name, address, buzzer_url) VALUES ("cwrkng-door", "https://door.cwrkng.de", "http://111.222.333.444");
