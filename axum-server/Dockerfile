FROM rust:1.93-slim

ARG REST_USER
ARG REST_USER_PASSWORD

WORKDIR /server

COPY Cargo.toml Cargo.lock .
COPY src src
COPY static-content static-content

RUN cargo install --path .

EXPOSE 5000
