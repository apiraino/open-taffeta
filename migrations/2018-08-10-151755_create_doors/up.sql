CREATE TABLE doors (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL UNIQUE,
    ring BOOLEAN NOT NULL DEFAULT 0,
    ring_ts INTEGER
);

INSERT INTO doors (name) VALUES ("front-door");
