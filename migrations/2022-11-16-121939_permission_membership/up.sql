CREATE TABLE permission_membership
(
    id         SERIAL PRIMARY KEY NOT NULL,
    permission INT REFERENCES permissions (id),
    user_id    INT REFERENCES users (id)
);
