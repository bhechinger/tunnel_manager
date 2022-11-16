-- Permissions tables
CREATE TABLE permissions
(
    id          SERIAL PRIMARY KEY NOT NULL,
    name        VARCHAR(32)        NOT NULL,
    description VARCHAR(256)       NOT NULL
);
