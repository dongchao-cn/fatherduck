CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');

CREATE TABLE person (
    name TEXT,
    current_mood mood
);

INSERT INTO person
VALUES ('Pedro', 'happy'), ('Mark', NULL), ('Pagliacci', 'sad'), ('Mr. Mackey', 'ok');

-- SELECT *
-- FROM person
-- WHERE current_mood = 'sad';
