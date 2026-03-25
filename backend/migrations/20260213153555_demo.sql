/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

-- Add migration script here
DROP TABLE IF EXISTS "counter";
CREATE TABLE "counters"
(
    id            SMALLINT PRIMARY KEY,
    counter_value INTEGER NOT NULL DEFAULT 0
);
