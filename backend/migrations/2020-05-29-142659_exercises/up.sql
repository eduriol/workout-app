-- Your SQL goes here
CREATE TABLE exercises (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  sets INTEGER NOT NULL,
  reps INTEGER NOT NULL
)