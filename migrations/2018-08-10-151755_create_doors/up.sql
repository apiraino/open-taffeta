CREATE TABLE doors (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    rung BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO doors (id, name, rung) VALUES (0, "front-door", 'f');
