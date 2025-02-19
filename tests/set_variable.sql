SET VARIABLE my_var = 30;
SELECT 20 + getvariable('my_var') AS total;

SET VARIABLE my_date = DATE '2018-07-13';
select getvariable('my_date') as my_date;

SET VARIABLE my_string = 'Hello world';
select getvariable('my_string') as my_date;

-- SET VARIABLE my_map = MAP {'k1': 10, 'k2': 20};
-- select getvariable('my_map') as my_date;

RESET VARIABLE my_var;
