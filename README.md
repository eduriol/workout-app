# Workout application

## Get and run application
You need to have a local PostgreSQL database named 'workouts', with all privileges granted to user postgres/password.
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app
cargo run
```

## Build Docker image
If you have a Postgres DB in localhost and port 5432: 
```
docker build --tag="workout-app:latest" .
```
If you have a Postgres DB in another host and port 5432:
```
docker build --tag="workout-app:latest" . --build-arg database=postgres://<user>:<password>@<host-ip>/workouts
```
## Execution
```
docker run -p 8000:8000 -it workout-app:latest
```