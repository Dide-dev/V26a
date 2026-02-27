CREATE EXTENSION citext;

-- TODO: indexes

CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    email         CITEXT UNIQUE NOT NULL,               -- emails are case-insensitive
    password_hash VARCHAR(255)  NOT NULL,               -- https://www.php.net/manual/en/function.password-hash.php
    name          VARCHAR(128)  NOT NULL,               -- current longest name in the database is 25 characters, might consider shortening this
    nickname      VARCHAR(128)           DEFAULT NULL,  -- current longest is 31 characters
    about_me      VARCHAR(160)           DEFAULT NULL,  -- current longest is 127 characters
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email_visible BOOLEAN       NOT NULL DEFAULT FALSE, -- default false because AVG (`emailvermelding` in prototype)
    language      CHAR(2)       NOT NULL DEFAULT 'nl'
    -- old database had some keys on email, name and nickname, they are left out because they seem unnecessary for now
);

-- the current database has a `plaatje` field that seems to only be used to check if an image exists? This won't work with multiple images anyway, so has been excluded.
CREATE TABLE trees
(
    id              SERIAL PRIMARY KEY,
    owner_id        INTEGER      NOT NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP, -- `timestamp` in prototype
    last_edited     TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published       BOOLEAN      NOT NULL DEFAULT FALSE,
    times_visited   BIGINT       NOT NULL DEFAULT 0,
    last_visited    TIMESTAMPTZ           DEFAULT NULL,              -- commonly NULL in prototype, can consider setting NOT NULL and filling the null values with last_edited
    title           VARCHAR(128) NOT NULL,
    description     VARCHAR(256)          DEFAULT NULL,
    book_name       VARCHAR(192)          DEFAULT NULL,
    book_edition    VARCHAR(32)           DEFAULT NULL,
    book_isbn       VARCHAR(13)           DEFAULT NULL,
    book_chapter    VARCHAR(32)           DEFAULT NULL,
    book_exercise   VARCHAR(32)           DEFAULT NULL,
    assignment_text VARCHAR(5000)         DEFAULT NULL,              -- currently stored as HTML with images in BASE64. We might want to consider Markdown or some other way of storing this.
    structure       JSONB         NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE   -- deletes the tree if the user is ever deleted, not sure if we ever want this, but NOT NULL would mess things up anyway
);

CREATE TABLE hints
(
    id       SERIAL PRIMARY KEY,
    front    VARCHAR(1024) DEFAULT NULL,
    back     VARCHAR(5000) DEFAULT NULL,
    tree_id  INTEGER       DEFAULT NULL, -- library cards are not linked to a tree
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE   -- deleting a tree deletes its cards
);
