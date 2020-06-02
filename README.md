# Workout application

## Build Docker image
If you have a Postgres DB in localhost and port 5432: 
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app
docker build --tag="workout-app:latest" .
```
## Execution
```
docker run -p 8000:8000 -it workout-app:latest
```