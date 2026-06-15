/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

-- tags should be case-insensitive on lookups
CREATE UNIQUE INDEX tags_lower ON tags (LOWER(name))