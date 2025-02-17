DETACH DATABASE IF EXISTS test;
ATTACH ':memory:' AS test;
USE test;

CREATE TABLE t1 (
    i INTEGER, 
    j INTEGER
);
