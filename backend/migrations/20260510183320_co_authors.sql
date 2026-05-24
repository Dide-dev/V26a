/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE co_authors(
    tree_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    PRIMARY KEY (tree_id, user_id),
    FOREIGN KEY (tree_id) REFERENCES trees (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
CREATE INDEX co_author_user_id ON co_authors (user_id);
