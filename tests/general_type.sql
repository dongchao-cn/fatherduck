-- https://duckdb.org/docs/sql/data_types/overview
CREATE TABLE t_BIGINT (
    a BIGINT,
    b INT8,
    c LONG,
);

INSERT INTO t_BIGINT VALUES (
    123::BIGINT,
    123::INT8,
    123::LONG,
);

select 
    a,
    b,
    c,
from t_BIGINT;


CREATE TABLE t_BLOB (
    a BLOB,
    b BYTEA,
    c BINARY,
    d VARBINARY,
);

INSERT INTO t_BLOB VALUES (
    '\xAA\xAB\xAC'::BLOB,
    '\xAA\xAB\xAC'::BYTEA,
    '\xAA\xAB\xAC'::BINARY,
    '\xAA\xAB\xAC'::VARBINARY,
);

select 
    hex(a),
    hex(b),
    hex(c),
    hex(d),
from t_BLOB;


CREATE TABLE t_BOOLEAN (
    a BOOLEAN,
    b BOOL,
    c LOGICAL,
);

INSERT INTO t_BOOLEAN VALUES (
    true::BOOLEAN,
    true::BOOLEAN,
    true::BOOLEAN,
);

select 
    a,
    b,
    c,
from t_BOOLEAN;


-- CREATE TABLE t2 (
--     a BIGINT,
--     b INT8,
--     c LONG,
--     -- b BIT,
--     c BLOB,
--     d BOOLEAN,
--     e DATE,
--     f INTEGER, 
--     g DECIMAL(10, 2),
-- );

-- INSERT INTO t2 VALUES (
--     123::BIGINT,
--     -- '101010'::BIT,
--     '\xAA\xAB\xAC'::BLOB,
--     true::BOOLEAN,
--     DATE '1992-09-20',
--     123::INTEGER, 
--     123.45::DECIMAL(10, 2)
-- );

-- select 
--     a,
--     -- b,
--     hex(c), 
--     d::text,
--     e,
--     f,
--     g,
-- from t2;