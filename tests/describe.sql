describe t1;
describe SELECT * FROM t1;
CREATE TABLE t1_description AS SELECT * FROM (DESCRIBE t1);
select * from t1_description;
