/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE classroom_tokens
(
    token TEXT PRIMARY KEY
);

CREATE TABLE classroom_sessions(
    token               TEXT        NOT NULL,
    classroom_id        INTEGER     NOT NULL, -- No ON DELETE CASCADE because it's handled by the foreign key
    id_within_classroom INTEGER     NOT NULL,
    expires_at          TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (token, classroom_id, id_within_classroom),
    FOREIGN KEY (token) REFERENCES classroom_tokens (token) ON DELETE CASCADE,
    FOREIGN KEY (classroom_id, id_within_classroom)
        REFERENCES classroom_users (classroom_id, id_within_classroom) ON DELETE CASCADE,
    CONSTRAINT token_classroom_id_unique UNIQUE (token, classroom_id)
);
