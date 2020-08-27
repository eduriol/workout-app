#!/bin/bash

# Set as env variable the password used by dropdb, createdb and psql to avoid prompting for it
export PGPASSWORD=password
# Remove the DB if existing and create it again
dropdb -U postgres --if-exists testing_workouts
createdb -U postgres testing_workouts
# Create the DB schema
diesel setup --database-url postgres://postgres:password@localhost/testing_workouts
# Import the data example that the tests expect
psql -U postgres -d testing_workouts -h localhost -a -f example_data.sql
# Remove the password from the env var
unset PGPASSWORD
