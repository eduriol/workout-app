-- This file should undo anything in `up.sql`
ALTER TABLE exercises
DROP COLUMN min_reps;
ALTER TABLE exercises
RENAME COLUMN max_reps TO reps;