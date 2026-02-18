-- Add migration script here
DROP TABLE IF EXISTS "counter";
CREATE TABLE "counters"
(
    id            SMALLINT PRIMARY KEY,
    counter_value INTEGER NOT NULL DEFAULT 0
);
