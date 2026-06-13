/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

-- We will optimize the searches for dutch text, since most of the current database is in dutch.
-- It is still possible to search for english terms, but there simply won't be as much normalization done.
ALTER TABLE trees ADD COLUMN search_vector tsvector GENERATED ALWAYS AS (to_tsvector('dutch', title || ' ' || description || ' ' || assignment_text)) STORED;

CREATE INDEX trees_search_idx ON trees USING GIN (search_vector);

-- This column with index can be used by doing for example:
-- SELECT * FROM trees WHERE websearch_to_tsquery('dutch', $1) @@ trees.search_vector
