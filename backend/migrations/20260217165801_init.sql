CREATE EXTENSION citext;

-- TODO: indexes

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email CITEXT UNIQUE NOT NULL, -- emails are case-insensitive
    password_hash VARCHAR(255) NOT NULL, -- https://www.php.net/manual/en/function.password-hash.php
    name VARCHAR(128) NOT NULL, -- current longest name in the database is 25 characters, might consider shortening this
    nickname VARCHAR(128) DEFAULT NULL, -- current longest is 31 characters
    about_me VARCHAR(160) DEFAULT NULL, -- current longest is 127 characters
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email_visible BOOLEAN NOT NULL DEFAULT FALSE, -- default false because AVG (`emailvermelding` in prototype)
    language CHAR(2) NOT NULL DEFAULT 'nl'
    -- old database had some keys on email, name and nickname, they are left out because they seem unnecessary for now
);

-- these unique constraints might make a moderation issue later, might want to reconsider
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name VARCHAR(192) DEFAULT NULL, -- contains the part (i.e. "Deel 1)
    edition VARCHAR(32) DEFAULT NULL,
    isbn VARCHAR(13) UNIQUE DEFAULT NULL, -- NULL should get treated as distinct, didn't test.
    UNIQUE(name, edition)
);

-- these unique constraints might make a moderation issue later, might want to reconsider
CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    chapter VARCHAR(32) DEFAULT NULL,
    book INTEGER NOT NULL,
    UNIQUE(chapter, book),
    FOREIGN KEY (book) REFERENCES books (id) ON DELETE CASCADE -- deleting a book deletes its chapters
);

-- these unique constraints might make a moderation issue later, might want to reconsider
CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    exercise VARCHAR(32) DEFAULT NULL,
    chapter INTEGER  NOT NULL,
    UNIQUE (exercise, chapter),
    FOREIGN KEY (chapter) REFERENCES chapters (id) ON DELETE CASCADE, -- deleting a chapter deletes its exercises
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE trees (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- `timestamp` in prototype
    published BOOLEAN NOT NULL DEFAULT FALSE,
    times_visited BIGINT NOT NULL DEFAULT 0,
    last_visited TIMESTAMP DEFAULT NULL,
    title VARCHAR(128) NOT NULL,
    description VARCHAR(256) DEFAULT NULL,
    exercise INTEGER DEFAULT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE, -- deletes the tree if the user is ever deleted, not sure if we ever want this, but NOT NULL would mess things up anyway
    FOREIGN KEY (exercise) REFERENCES exercises (id) ON DELETE SET NULL
);

-- the current database has a `plaatje` field that seems to only be used to check if an image exists? This won't work with multiple images anyway, so has been excluded.
CREATE TABLE tree_versions (
    id BIGSERIAL PRIMARY KEY,
    tree_id INTEGER NOT NULL,
    edited_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    structure JSON NOT NULL, -- TODO: default?
    assignment_text VARCHAR(5000) DEFAULT NULL, -- currently stored as HTML with images in BASE64. We might want to consider Markdown or some other way of storing this (JSON?).
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE -- deleting a tree should delete its history
);

CREATE TABLE hints (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    front VARCHAR(1024) DEFAULT NULL,
    back VARCHAR(5000) DEFAULT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE -- deleting a user deletes their cards
);

CREATE TABLE tree_hint (
    tree_id INTEGER NOT NULL,
    hint_id INTEGER NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees(id) ON DELETE CASCADE,
    FOREIGN KEY (hint_id) REFERENCES hints(id) ON DELETE CASCADE,
    CONSTRAINT tree_hint_pkey PRIMARY KEY (tree_id, hint_id)
);
