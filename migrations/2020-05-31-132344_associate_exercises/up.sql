-- Your SQL goes here
ALTER TABLE exercises
ADD COLUMN workout_id INTEGER,
ADD FOREIGN KEY (workout_id) REFERENCES workouts (id)