/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

ALTER TABLE lesson_trees
    DROP CONSTRAINT lesson_trees_tree_id_fkey;
ALTER TABLE lesson_trees
    DROP CONSTRAINT lesson_trees_lesson_id_fkey;

ALTER TABLE lesson_trees
    ADD CONSTRAINT lesson_trees_tree_id_fkey
        FOREIGN KEY (tree_id) REFERENCES trees (id)
            ON DELETE CASCADE;
ALTER TABLE lesson_trees
    ADD CONSTRAINT lesson_trees_lesson_id_fkey
        FOREIGN KEY (lesson_id) REFERENCES lessons (id)
            ON DELETE CASCADE;
