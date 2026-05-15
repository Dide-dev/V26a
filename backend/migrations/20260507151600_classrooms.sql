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
    name                    VARCHAR(32)         NOT NULL,
    --^ name is not UNIQUE because classroom names are not searchable or identified by their name.
    -- For students, the important item is the join_code, which does need to be UNIQUE.
    description             VARCHAR(256)        NOT NULL    DEFAULT '',
    joinable                BOOLEAN             NOT NULL    DEFAULT TRUE,
    join_code               VARCHAR(6) UNIQUE   NOT NULL,
    student_id_increment    INTEGER             NOT NULL    DEFAULT 1,
    --^ counter for the student id within the classroom, to allow IDs to stay low for the student
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);
