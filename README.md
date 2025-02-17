# fatherduck

## 准备libduckdb
cd ./libduckdb
curl -L -o libduckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.2.0/libduckdb-linux-amd64.zip
unzip libduckdb.zip -d ./

## 客户端
- [X] psql
- [X] dbeaver

## 语法适配
- [X] ANALYZE
- [X] ALTER TABLE `CREATE TABLE integers (i INTEGER, j INTEGER);`
    - [X] ALTER TABLE integers ADD COLUMN k INTEGER;
    - [X] ALTER TABLE integers DROP k;
    - [X] ALTER TABLE integers ALTER i TYPE VARCHAR;
    - [X] ALTER TABLE integers RENAME i TO ii;
    - [X] ALTER TABLE integers ADD PRIMARY KEY (i);
    - [X] ALTER TABLE integers RENAME TO integers_old;
    - [X] ALTER TABLE integers ALTER COLUMN i SET DEFAULT 10;
- [ ] ALTER VIEW
- [ ] ATTACH and DETACH
- [ ] CALL
- [ ] CHECKPOINT
- [ ] COMMENT ON
- [ ] COPY
- [ ] CREATE INDEX
- [ ] CREATE MACRO
- [ ] CREATE SCHEMA
- [ ] CREATE SECRET
- [ ] CREATE SEQUENCE
- [X] CREATE TABLE
    - [X] CREATE TABLE t1 (i INTEGER, j INTEGER);
    - [X] CREATE TABLE t1 AS SELECT 42 AS i, 84 AS j;
    - [X] CREATE TEMP TABLE t1 AS SELECT 42 AS i, 84 AS j;
    - [X] CREATE OR REPLACE TABLE t1 (i INTEGER, j INTEGER);
    - [X] CREATE OR REPLACE TABLE t1 AS SELECT 42 AS i, 84 AS j;
    - [X] CREATE OR REPLACE TEMP TABLE t1 AS SELECT 42 AS i, 84 AS j;
    - [X] CREATE TABLE IF NOT EXISTS t1 (i INTEGER, j INTEGER);
- [ ] CREATE VIEW
- [ ] CREATE TYPE
- [X] DELETE
- [ ] DESCRIBE
    - [X] DESCRIBE tbl;
    - [X] DESCRIBE SELECT * FROM tbl;
    - [ ] CREATE TABLE tbl_description AS SELECT * FROM (DESCRIBE tbl);
- [X] DROP
- [ ] EXPORT and IMPORT DATABASE
- [X] INSERT `CREATE TABLE t1 (i INTEGER);`
    - [X] INSERT INTO tbl VALUES (1), (2), (3);
    - [X] INSERT OR IGNORE INTO tbl (i) VALUES (1);
- [ ] LOAD / INSTALL
- [ ] PIVOT
- [ ] Profiling
- [X] SELECT
- [ ] SET / RESET
- [ ] SET VARIABLE
- [ ] SUMMARIZE
- [ ] Transaction Management
- [ ] UNPIVOT
- [X] UPDATE
- [ ] USE
- [ ] VACUUM

## 测试
```
show databases;
select 1;

PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name -c "SELECT 1;"
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name -c "describe weather; \bind \g"
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name -c 'describe weather \bind \g'
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name -f commands.sql > output.log


CREATE TABLE weather (
    city    VARCHAR,
    temp_lo INTEGER, -- minimum temperature on a day
    temp_hi INTEGER, -- maximum temperature on a day
    prcp    FLOAT,
    date    DATE
);
INSERT INTO weather
VALUES ('San Francisco', 46, 50, 0.25, '1994-11-27');
INSERT INTO weather (city, temp_lo, temp_hi, prcp, date)
VALUES ('San Francisco', 43, 57, 0.0, '1994-11-29');
select * from weather;
delete from weather where date = '1994-11-29';

SELECT * FROM generate_series(5);

-- 准备参数化查询
PREPARE my_query AS SELECT * FROM weather WHERE temp_lo = ?;

-- 执行查询并传递参数
EXECUTE my_query(46);

-- 释放资源
DEALLOCATE my_query;
```
