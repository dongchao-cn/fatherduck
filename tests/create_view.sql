CREATE VIEW v1 AS SELECT * FROM t1;
select * from v1;

CREATE OR REPLACE VIEW v2 AS SELECT 42;
select * from v2;

SELECT sql FROM duckdb_views() WHERE view_name = 'v1';
