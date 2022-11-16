-- Agent table
CREATE TABLE agents
(
    uuid  VARCHAR UNIQUE                NOT NULL,
    owner INTEGER REFERENCES users (id) NOT NULL
);
