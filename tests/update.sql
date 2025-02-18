UPDATE t1
SET i = 0
WHERE i = 1;

select * from t1;

CREATE OR REPLACE TABLE t2 AS
    SELECT 1 AS key, 'new value' AS value
    UNION ALL
    SELECT 2 AS key, 'new value 2' AS value;
select * from t2;
