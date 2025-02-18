USE memory;
DETACH DATABASE IF EXISTS test;
ATTACH ':memory:' AS test;
USE test;

CREATE TABLE t1 (
    i INTEGER PRIMARY KEY,
    j varchar
);
INSERT INTO t1
    VALUES (1, 'a'), (2, 'b'), (3, 'c');
