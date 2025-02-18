INSERT INTO t1 VALUES (4, 'd');

INSERT OR IGNORE INTO t1 (i, j) VALUES (1, '2');

INSERT OR REPLACE INTO t1 (i, j)
    VALUES (1, 'xxx');

select * from t1;
