CREATE TABLE roles (
       id INTEGER PRIMARY KEY NOT NULL,
       name VARCHAR NOT NULL,
       user INTEGER,
       FOREIGN KEY(user) REFERENCES users(id)
);
