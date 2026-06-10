-- tags should be case-insensitive on lookups
CREATE UNIQUE INDEX tags_lower ON tags (LOWER(name))