FROM rust:1.92-slim

WORKDIR /server

COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo install --path .

EXPOSE 5000
