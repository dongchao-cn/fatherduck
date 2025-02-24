CREATE TABLE t2 (
    a INTEGER, 
    -- b BIT,
    c BLOB,
    d BOOLEAN,
    e DATE,
);

INSERT INTO t2 VALUES (
    123::INTEGER, 
    -- '101010'::BITSTRING,
    '\xAA\xAB\xAC'::BLOB,
    true::BOOLEAN,
    DATE '1992-09-20',
);

select 
    a,
    -- b,
    hex(c), 
    d::text,
    e
from t2;
