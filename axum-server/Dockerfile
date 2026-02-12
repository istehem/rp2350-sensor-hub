FROM rust:1.93-slim

ARG ARCH_TARGET
ARG REST_USER
ARG REST_USER_PASSWORD

WORKDIR /server

COPY Cargo.toml Cargo.lock .
COPY src src
COPY static-content static-content

ENV TARGET=${ARCH_TARGET:-x86_64-unknown-linux-gnu}
RUN echo "Building the server for target $TARGET"
RUN if [ "$TARGET" = "aarch64-unknown-linux-gnu" ]; then rustup target add $TARGET; fi
RUN cargo install --target $TARGET --path .

EXPOSE 5000
