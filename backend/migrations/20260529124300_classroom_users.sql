/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE classroom_users
(
    id                      SERIAL PRIMARY KEY,
    classroom_id            INTEGER             NOT NULL,
    name                    VARCHAR(32)         NOT NULL,
    --^ name is once again not UNIQUE, not even within a classroom,
    -- because users are identified by their id_within_classroom in a classroom.
    id_within_classroom     INTEGER             NOT NULL,
    FOREIGN KEY (classroom_id) REFERENCES classrooms (id) ON DELETE CASCADE
);
