USE memory;
DETACH DATABASE IF EXISTS test;
ATTACH ':memory:' AS test;
USE test;

CREATE TABLE t1 (
    i INTEGER, 
    j INTEGER
);
INSERT INTO t1
    VALUES (1, 2), (3, 4), (5, 6);
