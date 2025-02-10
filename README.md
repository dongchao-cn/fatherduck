# fatherduck

## 准备libduckdb
cd ./libduckdb
curl -L -o libduckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.2.0/libduckdb-linux-amd64.zip
unzip libduckdb.zip -d ./


## 测试
show databases;
select 1;

PGPASSWORD='pencil' psql -h 127.0.0.1 -p 5432 -U username -d database_name
PGPASSWORD='pencil' psql -h 127.0.0.1 -p 5432 -U username -d database_name -c "SELECT 1;"
