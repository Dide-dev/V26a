/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE languages
(
    code CHAR(2)     PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO languages (code, name) VALUES ('nl', 'Nederlands'), ('en', 'English'), ('de', 'Deutsch'), ('fr', 'Français');

CREATE TABLE subjects
(
    id            SERIAL      PRIMARY KEY,
    language_code CHAR(2)     NOT NULL,
    name          VARCHAR(32) NOT NULL,
    UNIQUE (language_code, name),
    FOREIGN KEY (language_code) REFERENCES languages (code)
);

INSERT INTO subjects (language_code, name) VALUES ('nl', 'Wiskunde A'), ('nl', 'Wiskunde B'), ('nl', 'Wiskunde C'), ('nl', 'Wiskunde D'), ('nl', 'Nederlands'), ('nl', 'Engels');

CREATE TABLE books
(
    isbn    BIGINT       PRIMARY KEY,
    title   VARCHAR(192) NOT NULL DEFAULT '',
    author  VARCHAR(192) NOT NULL DEFAULT '',
    edition VARCHAR(32)  NOT NULL DEFAULT ''
);

CREATE TABLE tags
(
    id   SERIAL      PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

CREATE TABLE tree_tags
(
    tree_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE, -- deleting a tree also deletes its tags
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE, -- deleting a tag also removes it from all trees
    PRIMARY KEY (tree_id, tag_id)
);

CREATE INDEX tags_tree ON tree_tags (tree_id);
CREATE INDEX tags_tag ON tree_tags (tag_id);

ALTER TABLE users RENAME COLUMN language TO language_code;
ALTER TABLE users ADD CONSTRAINT fk_language_code FOREIGN KEY (language_code) REFERENCES languages (code);

ALTER TABLE trees DROP COLUMN book_name, DROP COLUMN book_edition, DROP COLUMN book_isbn;
ALTER TABLE trees ADD language_code CHAR(2) NOT NULL DEFAULT 'nl';
ALTER TABLE trees ADD CONSTRAINT fk_language_code FOREIGN KEY (language_code) REFERENCES languages (code);
ALTER TABLE trees ADD subject_id INTEGER DEFAULT NULL;
ALTER TABLE trees ADD CONSTRAINT fk_subject_id FOREIGN KEY (subject_id) REFERENCES subjects (id);
ALTER TABLE trees ADD book_isbn BIGINT DEFAULT NULL;
ALTER TABLE trees ADD CONSTRAINT fk_book_isbn FOREIGN KEY (book_isbn) REFERENCES books (isbn);
CREATE INDEX trees_filter_book_isbn ON trees (book_isbn);

UPDATE trees SET description = '' WHERE description IS NULL;
ALTER TABLE trees ALTER COLUMN description SET NOT NULL;
ALTER TABLE trees ALTER COLUMN description SET DEFAULT '';
UPDATE trees SET assignment_text = '' WHERE assignment_text IS NULL;
ALTER TABLE trees ALTER COLUMN assignment_text SET NOT NULL;
ALTER TABLE trees ALTER COLUMN assignment_text SET DEFAULT '';
UPDATE trees SET book_chapter = '' WHERE book_chapter IS NULL;
ALTER TABLE trees ALTER COLUMN book_chapter SET NOT NULL;
ALTER TABLE trees ALTER COLUMN book_chapter SET DEFAULT '';
UPDATE trees SET book_exercise = '' WHERE book_exercise IS NULL;
ALTER TABLE trees ALTER COLUMN book_exercise SET NOT NULL;
ALTER TABLE trees ALTER COLUMN book_exercise SET DEFAULT '';
