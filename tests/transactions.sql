SELECT * FROM t1;

BEGIN TRANSACTION;
INSERT INTO t1 VALUES (4, 'd');
COMMIT;
SELECT * FROM t1;

BEGIN TRANSACTION;
DELETE FROM t1 WHERE j = 'd';
INSERT INTO t1 VALUES (5, 'e');
ROLLBACK;
SELECT * FROM t1;

BEGIN TRANSACTION;
DELETE FROM t1 WHERE j = 'd';
INSERT INTO t1 VALUES (5, 'e');
ABORT;
SELECT * FROM t1;
