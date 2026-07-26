ARG PLATFORM=linux/amd64
ARG ARCH_TARGET=x86_64-unknown-linux-gnu

FROM rust:1.97-slim AS builder

ARG ARCH_TARGET
ARG REST_USER
ARG REST_USER_PASSWORD

WORKDIR /server

COPY Cargo.toml Cargo.lock .
COPY src src
COPY static-content static-content

RUN rustup target add $ARCH_TARGET && \
  if [ "${ARCH_TARGET}" = "aarch64-unknown-linux-gnu" ]; then \
    apt-get update && apt-get install -y --no-install-recommends \
      gcc-aarch64-linux-gnu \
      libc6-dev-arm64-cross; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-gcc"; \
  fi && \
  cargo build --release --target ${ARCH_TARGET}


FROM --platform=$PLATFORM gcr.io/distroless/cc-debian13:latest

ARG ARCH_TARGET

COPY --from=builder /server/target/$ARCH_TARGET/release/axum-server /usr/local/bin/

EXPOSE 5000
