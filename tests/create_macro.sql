CREATE MACRO add(a, b) AS a + b;
select add(1,2);

CREATE MACRO static_table() AS TABLE
    SELECT 'Hello' AS column1, 'World' AS column2;
select * from static_table();
