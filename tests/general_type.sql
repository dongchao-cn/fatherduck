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


CREATE TABLE t_DATE (
    a DATE,
);

INSERT INTO t_DATE VALUES (
    DATE '1992-09-20',
);

select 
    a,
from t_DATE;




CREATE TABLE t_DECIMAL (
    a DECIMAL(10, 2),
    b NUMERIC(10, 2),
);

INSERT INTO t_DECIMAL VALUES (
    123.45::DECIMAL(10, 2),
    123.45::NUMERIC(10, 2),
);

select 
    a,
    b,
from t_DECIMAL;



CREATE TABLE t_INTEGER (
    a INTEGER,
    b INT4,
    c INT,
    d SIGNED,

);

INSERT INTO t_INTEGER VALUES (
    123::INTEGER,
    123::INT4,
    123::INT,
    123::SIGNED,
);

select 
    a,
    b,
    c,
    d,
from t_INTEGER;



CREATE TABLE t_DOUBLE (
    a DOUBLE,
    b FLOAT8,

);

INSERT INTO t_DOUBLE VALUES (
    123.45::DOUBLE,
    123.45::FLOAT8,
);

select 
    a,
    b,
from t_DOUBLE;


CREATE TABLE t_FLOAT (
    a FLOAT,
    b FLOAT4,
    c REAL,
);

INSERT INTO t_FLOAT VALUES (
    123.45::FLOAT,
    123.45::FLOAT4,
    123.45::REAL,
);

select 
    a,
    b,
    c,
from t_FLOAT;

-- CREATE TABLE t_HUGEINT (
--     a HUGEINT,
-- );

-- INSERT INTO t_HUGEINT VALUES (
--     123::HUGEINT,
-- );

-- select 
--     a,
-- from t_HUGEINT;


SELECT cast(epoch(INTERVAL 1 YEAR) as integer);


CREATE TABLE t_SMALLINT (
    a SMALLINT,
    b INT2,
    c SHORT,
);

INSERT INTO t_SMALLINT VALUES (
    123::SMALLINT,
    123::INT2,
    123::SHORT,
);

select 
    a,
    b,
    c,
from t_SMALLINT;



CREATE TABLE t_TIME (
    a TIME,
);

INSERT INTO t_TIME VALUES (
    TIME '1992-09-20 11:30:00.123456',
);

select 
    a,
from t_TIME;



-- CREATE TABLE t_TIMETZ (
--     a TIMETZ,
-- );

-- INSERT INTO t_TIMETZ VALUES (
--     TIMETZ '1992-09-20 11:30:00.123456-02:00';
-- );

-- select 
--     a,
-- from t_TIMETZ;

CREATE TABLE t_TIMESTAMP (
    a TIMESTAMP,
    b DATETIME,
);

INSERT INTO t_TIMESTAMP VALUES (
    TIMESTAMP '1992-09-20 11:30:00.123456789',
    DATETIME '1992-09-20 11:30:00.123456789',
);

select 
    a,
    b,
from t_TIMESTAMP;



CREATE TABLE t_TINYINT (
    a TINYINT,
    b INT1,
);

INSERT INTO t_TINYINT VALUES (
    1::TINYINT,
    1::INT1,
);

select 
    a,
    b,
from t_TINYINT;



CREATE TABLE t_UINTEGER (
    a UINTEGER,
);

INSERT INTO t_UINTEGER VALUES (
    1::UINTEGER,
);

select 
    a,
from t_UINTEGER;


-- VARCHAR, CHAR, BPCHAR, TEXT, STRING
CREATE TABLE t_VARCHAR (
    a VARCHAR,
    b CHAR,
    c BPCHAR,
    d TEXT,
    e STRING,
);

INSERT INTO t_VARCHAR VALUES (
    'abc'::VARCHAR,
    'abc'::CHAR,
    'abc'::BPCHAR,
    'abc'::TEXT,
    'abc'::STRING,
);

select 
    a,
    b,
    c,
    d,
    e,
from t_VARCHAR;

select 'a050f680-dffd-4aaa-be89-1e557aa51171'::UUID as uuid;

