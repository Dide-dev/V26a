/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE classrooms
(
    id                      SERIAL PRIMARY KEY,
    owner_id                INTEGER             NOT NULL,
    created_at              TIMESTAMPTZ         NOT NULL    DEFAULT CURRENT_TIMESTAMP,
    name                    VARCHAR(32)         NOT NULL, -- name is not UNIQUE by design
    description             VARCHAR(256)        NOT NULL    DEFAULT '',
    joinable                BOOLEAN             NOT NULL    DEFAULT TRUE,
    join_code               VARCHAR(6) UNIQUE   NOT NULL,
    student_id_increment    INTEGER             NOT NULL    DEFAULT 1,
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);
