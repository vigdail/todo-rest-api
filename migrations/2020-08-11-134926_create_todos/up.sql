-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body VARCHAR,
    completed BOOLEAN NOT NULL DEFAULT 'f'
)