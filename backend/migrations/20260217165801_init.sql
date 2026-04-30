/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE EXTENSION citext;

-- TODO: indexes

CREATE TABLE languages
(
    code CHAR(2)     PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    email         CITEXT UNIQUE NOT NULL,               -- emails are case-insensitive
    password_hash VARCHAR(255)  NOT NULL,               -- https://www.php.net/manual/en/function.password-hash.php
    enabled       BOOLEAN       NOT NULL DEFAULT TRUE,  -- if set to false, this user cannot log in
    name          VARCHAR(128)  NOT NULL,               -- current longest name in the database is 25 characters, might consider shortening this
    nickname      VARCHAR(128)  NOT NULL DEFAULT '',  -- current longest is 31 characters
    about_me      VARCHAR(160)  NOT NULL DEFAULT '',  -- current longest is 127 characters
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email_visible BOOLEAN       NOT NULL DEFAULT FALSE, -- default false because AVG (`emailvermelding` in prototype)
    account_type  VARCHAR(32)   NOT NULL DEFAULT 'author', -- default is author
    language_code CHAR(2)       NOT NULL DEFAULT 'nl',   -- ISO 639-1
    -- old database had some keys on email, name and nickname, they are left out because they seem unnecessary for now
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

CREATE TABLE subjects
(
    id            SERIAL      PRIMARY KEY,
    language_code CHAR(2)     NOT NULL,
    name          VARCHAR(32) NOT NULL,
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

CREATE TABLE books
(
    isbn    VARCHAR(13)  PRIMARY KEY,
    title   VARCHAR(192) DEFAULT NULL,
    author  VARCHAR(192) DEFAULT NULL,
    edition VARCHAR(32)  DEFAULT NULL
);

-- the current database has a `plaatje` field that seems to only be used to check if an image exists? This won't work with multiple images anyway, so has been excluded.
CREATE TABLE trees
(
    id              SERIAL PRIMARY KEY,
    owner_id        INTEGER      NOT NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP, -- `timestamp` in prototype
    last_modified   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published       BOOLEAN      NOT NULL DEFAULT FALSE,
    times_visited   BIGINT       NOT NULL DEFAULT 0,
    last_visited    TIMESTAMPTZ           DEFAULT NULL,              -- commonly NULL in prototype, can consider setting NOT NULL and filling the null values with last_edited
    title           VARCHAR(128) NOT NULL,
    language_code   CHAR(2)      NOT NULL,
    description     VARCHAR(256)          DEFAULT NULL,
    subject_id      INTEGER               DEFAULT NULL,
    book_isbn       VARCHAR(13)           DEFAULT NULL,
    book_chapter    VARCHAR(32)           DEFAULT NULL,
    book_exercise   VARCHAR(32)           DEFAULT NULL,
    assignment_text VARCHAR(5000)         DEFAULT NULL,              -- currently stored as HTML with images in BASE64. We might want to consider Markdown or some other way of storing this.
    structure       JSONB        NOT NULL,
    FOREIGN KEY (language_code) REFERENCES languages (code),
    FOREIGN KEY (subject_id) REFERENCES subjects (id),
    FOREIGN KEY (book_isbn) REFERENCES books (isbn),
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE   -- deletes the tree if the user is ever deleted, not sure if we ever want this, but NOT NULL would mess things up anyway
);

CREATE INDEX trees_filter_owner ON trees (owner_id);
CREATE INDEX trees_filter_order_created_at ON trees (created_at);
CREATE INDEX trees_filter_book_isbn ON trees (book_isbn);

CREATE TABLE tags
(
    tree_id INTEGER     NOT NULL,
    tag     VARCHAR(32) NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE, -- deleting a tree also deletes its tags
    UNIQUE (tree_id, tag)
);

CREATE INDEX tags_tree ON tags (tree_id);
CREATE INDEX tags_tag ON tags (tag);

CREATE TABLE cards
(
    id              SERIAL PRIMARY KEY,
    tree_id         INTEGER       NOT NULL,                               -- library cards will be their own table later(?), with foreign key to hints(id)
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    front           VARCHAR(1024) NOT NULL DEFAULT '',                    -- a lot of cards currently are empty at the front, TODO: check if this is done because of bad design or if it is actually desired.
    back            VARCHAR(5000) NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE -- deleting a tree deletes its cards
);

CREATE TABLE images
(
    hash     TEXT PRIMARY KEY,
    owner_id INTEGER     NOT NULL,
    filename TEXT        NOT NULL UNIQUE,
    added_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
