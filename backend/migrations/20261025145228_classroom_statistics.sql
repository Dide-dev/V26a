CREATE TABLE classroom_tree_usage_sessions
(
    -- Classroom user and tree relation.
    classroom_id      INTEGER     NOT NULL,
    classroom_user_id INTEGER     NOT NULL,
    tree_id           INTEGER     NOT NULL,

    PRIMARY KEY (classroom_id, classroom_user_id, tree_id),
    FOREIGN KEY (classroom_id, classroom_user_id) REFERENCES classroom_users (classroom_id, id_within_classroom) ON DELETE CASCADE,
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE,

    -- Usage statistics
    times_opened      INTEGER     NOT NULL DEFAULT 1,
    first_opened_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE classroom_cards_usages
(
    -- Classroom user and tree relation.
    classroom_id      INTEGER     NOT NULL,
    classroom_user_id INTEGER     NOT NULL,
    card_id           INTEGER     NOT NULL,

    PRIMARY KEY (classroom_id, classroom_user_id, card_id),
    FOREIGN KEY (classroom_id, classroom_user_id) REFERENCES classroom_users (classroom_id, id_within_classroom) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards (id) ON DELETE CASCADE,

    -- Usage statistics
    first_opened_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
