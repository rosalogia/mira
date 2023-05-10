-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    img_path VARCHAR,
    title VARCHAR,
    source VARCHAR,
    posted_at TIMESTAMP,
    score INTEGER NOT NULL
);