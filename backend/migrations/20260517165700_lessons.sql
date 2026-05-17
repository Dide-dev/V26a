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
    description         VARCHAR(256)    NOT NULL    DEFAULT '',
    published           BOOLEAN         NOT NULL    DEFAULT FALSE,
    publish_by          TIMESTAMPTZ                 DEFAULT NULL,
    --^ This is to allow for a scheduled deployment of lessons.
    -- Would there be need for a depublish_by?

    FOREIGN KEY (classroom_id) REFERENCES classrooms (id) ON DELETE CASCADE
);

CREATE TABLE classroom_lessons
(
    classroom_id        INTEGER,
    lesson_id           INTEGER,
    PRIMARY KEY (classroom_id, lesson_id),
    FOREIGN KEY (classroom_id)      REFERENCES classrooms (id),
    FOREIGN KEY (lesson_id)         REFERENCES lessons (id)
)