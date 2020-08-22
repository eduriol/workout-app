FROM rust:latest

WORKDIR /usr/src/app
COPY . .

# Uncomment before building image to easily launch the application if you use Docker Desktop for Mac
ENV DB_HOST=host.docker.internal

RUN rustup default nightly

RUN cargo install --path .

CMD ["workout-backend"]
