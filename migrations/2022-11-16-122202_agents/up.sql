-- Agent table
CREATE TABLE agents
(
    id    SERIAL PRIMARY KEY,
    uuid  VARCHAR UNIQUE                NOT NULL,
    owner INTEGER REFERENCES users (id) NOT NULL
);
