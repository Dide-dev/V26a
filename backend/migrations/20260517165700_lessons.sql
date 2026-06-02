/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE lessons
(
    id                  SERIAL PRIMARY KEY,
    classroom_id        INTEGER         NOT NULL,
    order_number        INTEGER UNIQUE  NOT NULL,
    created_at          TIMESTAMPTZ     NOT NULL    DEFAULT CURRENT_TIMESTAMP,
    name                VARCHAR(32)     NOT NULL,
    description         VARCHAR(128)    NOT NULL    DEFAULT '',
    published           BOOLEAN         NOT NULL    DEFAULT FALSE,

    FOREIGN KEY (classroom_id) REFERENCES classrooms (id) ON DELETE CASCADE
);

CREATE TABLE lesson_trees
(
    lesson_id        INTEGER,
    tree_id           INTEGER,
    PRIMARY KEY (lesson_id, tree_id),
    FOREIGN KEY (lesson_id)      REFERENCES lessons (id),
    FOREIGN KEY (tree_id)        REFERENCES trees (id)
);