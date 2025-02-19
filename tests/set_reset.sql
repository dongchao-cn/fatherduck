SET threads = 1;
SET threads TO 1;
SELECT current_setting('threads') as threads;
RESET threads;
SELECT current_setting('threads') as threads;

SET GLOBAL default_order = 'DESC';
SELECT * FROM duckdb_settings() WHERE name = 'default_order';
RESET GLOBAL default_order;
