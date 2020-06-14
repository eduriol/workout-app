FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN rustup default nightly

RUN cargo install --path .

CMD ["workout-app"]
