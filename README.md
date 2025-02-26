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

## 类型
https://duckdb.org/docs/sql/data_types/overview
- [ ] [General-Purpose Data Types](tests/general_type.sql)
    - [X] BIGINT, INT8, LONG
    - [ ] BIT, BITSTRING
    - [X] BLOB, BYTEA, BINARY, VARBINARY
    - [X] BOOLEAN, BOOL, LOGICAL
    - [X] DATE
    - [X] DECIMAL(prec, scale), NUMERIC(prec, scale)
    - [X] DOUBLE, FLOAT8
    - [X] FLOAT, FLOAT4, REAL
    - [ ] HUGEINT
    - [X] INTEGER, INT4, INT, SIGNED
    - [X] INTERVAL
    - [ ] JSON
    - [X] SMALLINT, INT2, SHORT
    - [X] TIME
    - [ ] TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ
    - [X] TIMESTAMP, DATETIME
    - [X] TINYINT, INT1
    - [ ] UBIGINT
    - [ ] UHUGEINT
    - [X] UINTEGER
    - [ ] USMALLINT
    - [ ] UTINYINT
    - [X] UUID
    - [X] VARCHAR, CHAR, BPCHAR, TEXT, STRING
- [ ] [Nested / Composite Types](tests/nested_type.sql)
    - [ ] ARRAY
    - [ ] LIST
    - [ ] MAP
    - [ ] STRUCT
    - [ ] UNION


## 语法
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
- [ ] [CREATE TYPE](tests/create_type.sql)
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
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name
PGPASSWORD='fatherduck' psql -h 127.0.0.1 -p 5432 -U fatherduck -d database_name -c "SELECT 1;"
```
