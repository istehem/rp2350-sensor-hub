export MEASUREMENTS_ENDPOINT := "http://192.168.132.170:5000/api/measurements"

set dotenv-required
set dotenv-path := "hub.env"

# default
default: pre-push

# check for outdated dependencies
outdated:
  # run `cargo upgrade --dry-run` to check versions defined in [workspace.dependencies]
  cargo outdated -w --root-deps-only

# run 'npm run {{COMMAND}}' on the frontend
frontend COMMAND:
  npm --prefix axum-server/measurements-frontend run {{COMMAND}}

# format code for rp2350
fmt-pico:
  cargo fmt

# format code for the server
fmt-server:
  cargo fmt --manifest-path ./axum-server/Cargo.toml

# run all tests
test-all:
  cargo test --target=x86_64-unknown-linux-gnu -p tests -- --nocapture

# TEST := die|game|player
test TEST:
  cargo test --target=x86_64-unknown-linux-gnu -p tests --test test-{{TEST}} -- --nocapture

# build code for rp2350
build-all-pico:
  cargo build --all --features temperature

# build for rp2350; no temperature feature
build-all-no-temperature-pico:
  cargo build --all

# lint code for rp2350
clippy-all-pico:
  cargo clippy --all --features temperature

# lint code for rp2350; no temperature feature
clippy-all-no-temperature-pico:
  cargo clippy --all

# build the server
build-server:
  cargo build --manifest-path ./axum-server/Cargo.toml --target=x86_64-unknown-linux-gnu

# build the server podman image
build-server-image:
  podman compose -f ./axum-server/docker-compose.yaml build

# lint the server
clippy-server:
  cargo clippy --manifest-path ./axum-server/Cargo.toml --target=x86_64-unknown-linux-gnu

# deploy and run the code on pico; debug probe required
run-pico:
  cargo run --release --features temperature

# the same as run-pico but no temperature feature
run-pico-no-temperature:
  cargo run --release

# use before git push
pre-push: \
  fmt-pico \
  build-all-pico \
  build-all-no-temperature-pico \
  clippy-all-pico \
  clippy-all-no-temperature-pico \
  fmt-server \
  build-server \
  clippy-server \
  (frontend 'install-deps') \
  (frontend 'format') \
  (frontend 'lint') \
  (frontend 'build')
