FROM rust:latest

WORKDIR /usr/src/app
COPY . .

ARG database="postgres://postgres:password@host.docker.internal/workouts"

ENV DATABASE_URL=$database

RUN rustup default nightly

RUN cargo install --path .

CMD ["workout-app"]
