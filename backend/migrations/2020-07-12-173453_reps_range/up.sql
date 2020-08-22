-- Your SQL goes here
ALTER TABLE exercises
ADD COLUMN min_reps INTEGER NOT NULL default 1;
ALTER TABLE exercises
RENAME COLUMN reps TO max_reps;