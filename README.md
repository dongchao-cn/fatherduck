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
- [X] ALTER TABLE
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
- [ ] CREATE VIEW
- [ ] CREATE TYPE
- [X] DELETE
- [ ] DESCRIBE
- [X] DROP
- [ ] EXPORT and IMPORT DATABASE
- [X] INSERT
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
