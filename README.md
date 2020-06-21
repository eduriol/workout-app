# Workout application
## Requirements
- [Rust](https://www.rust-lang.org/tools/install) nightly release.
- [PostgreSQL](https://www.postgresql.org/) 12+ database.
## Database
Workout application needs to connect to an existing PostgreSQL database.  
For the application to be able to connect to the database, create the following environment variables in your Operating System:
```
export DB_USER=<username>
export DB_PASS=<password>
export DB_HOST=<database_host>
export DB_NAME=<database_name>
```
If you don't specify any environment variable the application will try to connect with the following connection string:
```
postgres://postgres:password@localhost/workouts
```
### Example data
For development and debugging purposes, a *example_data.sql* file is provided.  
Workout app uses [pg_dump](https://www.postgresql.org/docs/current/app-pgdump.html) to generate this file.  
The file is generated with the following command:
```
pg_dump -U <username> -h <database_host> -a -T public.__diesel_schema_migrations <database_name> >> example_data.sql
```
To import this data into your database, use the following command:
```
psql -U <username> -d <database_name> -h <database_host> -a -f example_data.sql
```
## Run the application
As usual, first step is cloning the repo :)
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app
```
### Local execution
```
cargo run
```
And then access http://localhost:8000 to see the Workout application greeting.
### Docker execution
#### Build Docker image
```
docker build --tag="workout-app:latest" .
```
If you use Docker Desktop for Mac, see note in *Dockerfile*.
#### Run Docker image
```
docker run -p 8000:8000 -it workout-app:latest
```
And then access http://localhost:8000 to see the Workout application greeting.
### Google Kubernetes Engine deployment
The file *workout-app-gke-deployment.yml* contains the specification to deploy the application in [GKE](https://cloud.google.com/kubernetes-engine).  
Most of the content of the file is common Kubernetes configuration, but for information regarding more advanced stuff like the usage of database secrets as environment variables, set-up of service account and the proxy container, read [Connecting from Google Kubernetes Engine](https://cloud.google.com/sql/docs/mysql/connect-kubernetes-engine). 