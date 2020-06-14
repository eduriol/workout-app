FROM rust:latest

WORKDIR /usr/src/app
COPY . .

ENV DB_USER="postgres"

ENV DB_PASS="password"

ENV DB_HOST="host.docker.internal"

ENV DB_NAME="workouts"

RUN rustup default nightly

RUN cargo install --path .

CMD ["workout-app"]
