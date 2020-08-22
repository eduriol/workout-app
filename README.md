# Workout app
For the sake of avoiding code duplication, both the backend and frontend of the Workout application have been included in the same repository.  
You can find below the instructions to run both separately. 
## Workout app backend
### Backend requirements
- [Rust](https://www.rust-lang.org/tools/install) nightly release.
- [PostgreSQL](https://www.postgresql.org/) 12+ database.
- [Diesel CLI](http://diesel.rs/guides/getting-started/)
### Database
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
#### Setup database
After you have created the required PostgreSQL database, you need to import the latest schema version with the following command:
```
diesel migration run --database-url=postgres://<username>:<password>@<database_host>/<database_name>
```
#### Example data
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
### Run the backend
As usual, first step is cloning the repo :)
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app/backend
```
#### Local execution
```
cargo run
```
And then access http://localhost:8000 to see the Workout application greeting.
#### Docker execution
##### Build Docker image
```
docker build --tag="workout-backend:latest" .
```
If you use Docker Desktop for Mac, see note in *Dockerfile*.
##### Run Docker image
```
docker run -p 8000:8000 -it workout-backend:latest
```
And then access http://localhost:8000 to see the Workout application greeting.
#### Google Kubernetes Engine deployment
The file *workout-app-gke-deployment.yml* contains the specification to deploy the application in [GKE](https://cloud.google.com/kubernetes-engine).  
Most of the content of the file is common Kubernetes configuration, but for information regarding more advanced stuff like the usage of database secrets as environment variables, set-up of service account and the proxy container, read [Connecting from Google Kubernetes Engine](https://cloud.google.com/sql/docs/mysql/connect-kubernetes-engine).
## Workout app frontend
### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)  

By default, the Workout frontend expects the Workout backend to be running on http://localhost:8000.

### Run the frontend
As usual, first step is cloning the repo :)
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app/frontend
```
Then build the web into the 'static' directory and run the index with your favorite server (I use miniserve for local testing purposes):
```
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html
```
You can access http://localhost:8080 to see the Workout frontend.
 