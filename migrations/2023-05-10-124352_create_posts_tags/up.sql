-- Your SQL goes here
CREATE TABLE posts_tags (
    post_id INTEGER REFERENCES posts(id),
    tag_id INTEGER REFERENCES tags(id),
    PRIMARY KEY(post_id, tag_id)
);