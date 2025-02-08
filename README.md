# fatherduck

# 准备libduckdb
cd ./libduckdb
curl -L -o libduckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.2.0/libduckdb-linux-amd64.zip
unzip libduckdb.zip -d ./


# 
psql -h 127.0.0.1 -p 5432 -U username -d database_name

DUCKDB_LIB_DIR=/home/dongchao/duckdb DUCKDB_INCLUDE_DIR=/home/dongchao/duckdb cargo run -vv