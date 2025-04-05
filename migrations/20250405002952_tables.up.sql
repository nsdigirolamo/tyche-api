CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
    id UUID DEFAULT gen_random_uuid(),
    username VARCHAR(16) NOT NULL,
    password VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),
    UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS posts (
    id UUID DEFAULT gen_random_uuid(),
    parent_id UUID NULL,
    author_id UUID NOT NULL,
    body VARCHAR(80) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),
    FOREIGN KEY (parent_id) REFERENCES posts(id),
    FOREIGN KEY (author_id) REFERENCES users(id)
);
