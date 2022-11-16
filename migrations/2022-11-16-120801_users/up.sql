-- Create user table
CREATE TABLE users
(
    id    SERIAL PRIMARY KEY NOT NULL,
    email VARCHAR UNIQUE     NOT NULL
);
