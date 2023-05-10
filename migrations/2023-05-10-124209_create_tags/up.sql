-- Your SQL goes here
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    category INTEGER NOT NULL
);