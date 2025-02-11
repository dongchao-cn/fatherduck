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

SELECT * FROM generate_series(5);
