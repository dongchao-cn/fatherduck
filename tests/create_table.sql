CREATE TABLE t2 (i INTEGER, j INTEGER)
CREATE TABLE t3 AS SELECT 42 AS i, 84 AS j
-- CREATE TEMP TABLE t4 AS SELECT 42 AS i, 84 AS j -- todo
CREATE OR REPLACE TABLE t5 (i INTEGER, j INTEGER)
CREATE OR REPLACE TABLE t6 AS SELECT 42 AS i, 84 AS j
CREATE OR REPLACE TEMP TABLE t7 AS SELECT 42 AS i, 84 AS j
CREATE TABLE IF NOT EXISTS t8 (i INTEGER, j INTEGER)
