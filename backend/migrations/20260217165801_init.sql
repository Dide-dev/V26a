/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE EXTENSION citext;

CREATE TABLE languages
(
    code CHAR(2) PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO languages (code, name)
VALUES ('nl', 'Nederlands'),
       ('en', 'English'),
       ('de', 'Deutsch'),
       ('fr', 'Français');

CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    email         CITEXT UNIQUE NOT NULL,              -- emails are case-insensitive, endpoints enforce a 128-character limit
    password_hash VARCHAR(255)  NOT NULL,              -- https://github.com/Keats/rust-bcrypt/
    enabled       BOOLEAN       NOT NULL DEFAULT TRUE, -- whether the user can log in
    name          VARCHAR(32)   NOT NULL,
    nickname      VARCHAR(32)   NOT NULL DEFAULT '',
    about_me      VARCHAR(512)  NOT NULL DEFAULT '',
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email_visible BOOLEAN       NOT NULL DEFAULT FALSE,
    account_type  VARCHAR(32)   NOT NULL DEFAULT 'author',
    language_code CHAR(2)       NOT NULL DEFAULT 'nl',  -- ISO 639-1
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

CREATE TABLE subjects
(
    id            SERIAL PRIMARY KEY,
    language_code CHAR(2)     NOT NULL,
    name          VARCHAR(32) NOT NULL,
    UNIQUE (language_code, name),
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

INSERT INTO subjects (language_code, name)
VALUES ('nl', 'Wiskunde'),
       ('nl', 'Wiskunde A'),
       ('nl', 'Wiskunde B'),
       ('nl', 'Wiskunde C'),
       ('nl', 'Wiskunde D'),
       ('nl', 'Nederlands'),
       ('nl', 'Engels');

CREATE TABLE books
(
    isbn    BIGINT PRIMARY KEY,
    title   VARCHAR(192) NOT NULL DEFAULT '',
    author  VARCHAR(192) NOT NULL DEFAULT '',
    edition VARCHAR(32)  NOT NULL DEFAULT ''
);

-- username is case-insensitive for login
CREATE UNIQUE INDEX name_lower on users (LOWER(name));

CREATE TABLE trees
(
    id              SERIAL PRIMARY KEY,
    owner_id        INTEGER       NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified   TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published       BOOLEAN       NOT NULL DEFAULT FALSE,
    times_visited   BIGINT        NOT NULL DEFAULT 0,
    last_visited    TIMESTAMPTZ            DEFAULT NULL,
    title           VARCHAR(100)  NOT NULL,
    description     VARCHAR(500)  NOT NULL DEFAULT '',
    book_chapter    VARCHAR(32)   NOT NULL DEFAULT '',
    book_exercise   VARCHAR(32)   NOT NULL DEFAULT '',
    assignment_text VARCHAR(5000) NOT NULL DEFAULT '',
    structure       JSONB         NOT NULL,
    language_code   CHAR(2)       NOT NULL DEFAULT 'nl',
    subject_id      INTEGER                DEFAULT NULL,
    book_isbn       BIGINT                 DEFAULT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (language_code) REFERENCES languages (code),
    FOREIGN KEY (subject_id) REFERENCES subjects (id),
    FOREIGN KEY (book_isbn) REFERENCES books (isbn)
);

CREATE INDEX trees_filter_owner ON trees (owner_id);
CREATE INDEX trees_filter_order_created_at ON trees (created_at);
CREATE INDEX trees_filter_book_isbn ON trees (book_isbn);

CREATE TABLE tags
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

CREATE TABLE tree_tags
(
    tree_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE, -- deleting a tree also deletes its tags
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,   -- deleting a tag also removes it from all trees
    PRIMARY KEY (tree_id, tag_id)
);

CREATE INDEX tags_tree ON tree_tags (tree_id);
CREATE INDEX tags_tag ON tree_tags (tag_id);

CREATE TABLE cards
(
    id            SERIAL PRIMARY KEY,
    tree_id       INTEGER       NOT NULL,
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    front         VARCHAR(250)  NOT NULL DEFAULT '',
    back          VARCHAR(5000) NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE
);

CREATE TABLE images
(
    hash     TEXT PRIMARY KEY,
    owner_id INTEGER     NOT NULL,
    filename TEXT UNIQUE NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE library_cards
(
    id         SERIAL PRIMARY KEY,
    owner_id   INTEGER       NOT NULL,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title      VARCHAR(100)  NOT NULL,
    tags       VARCHAR(32)[],
    front      VARCHAR(250)  NOT NULL,
    back       VARCHAR(5000) NOT NULL
);

CREATE TABLE feedback
(
    id         SERIAL PRIMARY KEY,
    email      CITEXT        NOT NULL DEFAULT '',
    feedback   VARCHAR(5000) NOT NULL,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP
);
