/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

CREATE TABLE library_cards_tags
(
    library_card_id INTEGER NOT NULL,
    tag_id          INTEGER NOT NULL,
    FOREIGN KEY (library_card_id) REFERENCES library_cards (id) ON DELETE CASCADE, -- deleting a library_card also deletes its tags
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,   -- deleting a tag also removes it from all library_cards
    PRIMARY KEY (library_card_id, tag_id)
);

CREATE INDEX tags_library_cards ON library_cards_tags (library_card_id);
CREATE INDEX tags_tag_library_cards ON library_cards_tags (tag_id);

ALTER TABLE library_cards DROP COLUMN tags;