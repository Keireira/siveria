# Stage 1: Build Stage
FROM rust:1.82-bookworm AS build-stage

RUN apt update && \
  apt install -y --no-install-recommends \
  libssl-dev \
  musl-tools \
  libpq-dev

COPY . /app
WORKDIR /app

RUN cargo build --release

# Stage 2
FROM debian:bookworm-slim

COPY ./migrations ./migrations
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install libpq5 -y
COPY --from=build-stage /app/target/release/siveria /usr/bin

CMD ["siveria"]
EXPOSE 1338