# fatherduck

## 准备duckdb/libduckdb
cd ./libduckdb
curl -L -o duckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.2.0/duckdb_cli-linux-amd64.zip
unzip duckdb.zip -d ./
curl -L -o libduckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.2.0/libduckdb-linux-amd64.zip
unzip libduckdb.zip -d ./

## 客户端
- [X] psql
- [X] dbeaver

## 语法适配
- [X] [ANALYZE](tests/analyze.sql)
- [X] [ALTER TABLE](tests/alter_table.sql)
- [X] [ALTER VIEW](tests/alter_view.sql)
- [X] [ATTACH and DETACH](tests/attach_use_detach.sql)
- [X] [CALL](tests/call.sql)
- [X] [CHECKPOINT](tests/checkpoint.sql)
- [X] [COMMENT ON](tests/comment_on.sql)
- [ ] COPY
- [X] [CREATE INDEX](tests/create_index.sql)
- [X] [CREATE MACRO](tests/create_macro.sql)
- [X] [CREATE SCHEMA](tests/create_schema.sql)
- [ ] CREATE SECRET
- [X] [CREATE SEQUENCE](tests/create_sequence.sql)
- [X] [CREATE TABLE](tests/create_table.sql)
- [X] [CREATE VIEW](tests/create_view.sql)
- [ ] CREATE TYPE
- [X] [DELETE](tests/delete.sql)
- [X] [DESCRIBE](tests/describe.sql)
- [X] [DROP](tests/drop.sql)
- [ ] EXPORT and IMPORT DATABASE
- [X] [INSERT](tests/insert.sql)
- [ ] LOAD / INSTALL
- [ ] [PIVOT](tests/povit.sql) `todo` https://github.com/duckdb/duckdb/issues/7720
- [ ] [Profiling](tests/profiling.sql) `todo`
- [X] [SELECT](tests/select.sql)
- [X] [SET / RESET](tests/set_reset.sql)
- [X] [SET VARIABLE](tests/set_variable.sql)
- [ ] [SUMMARIZE](tests/summarize.sql) `todo`
- [X] [Transaction Management](tests/transactions.sql)
- [X] [UNPIVOT](tests/unpovit.sql)
- [X] [UPDATE](tests/update.sql)
- [X] [USE](tests/attach_use_detach.sql)
- [X] [VACUUM](tests/vacuum.sql)

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
