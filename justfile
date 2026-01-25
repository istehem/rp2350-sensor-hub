export MEASUREMENTS_ENDPOINT := "http://192.168.132.170:5000/api/measurements"

set dotenv-required
set dotenv-path := "hub.env"

# alias for pre-push
[group: 'check']
default: pre-push

# check for outdated dependencies
[group: 'maintenance']
outdated:
  # run `cargo upgrade --dry-run` to check versions defined in [workspace.dependencies]
  cargo outdated -w --root-deps-only

# run 'npm run {{COMMAND}}' on the frontend
[group: 'frontend']
frontend COMMAND:
  npm --prefix axum-server/measurements-frontend run {{COMMAND}}

# stage the frontend build
[group: 'frontend']
stage-frontend: (frontend 'install-deps') (frontend 'build') (frontend 'lint')
  #!/usr/bin/env bash
  echo '{{BOLD}}Cleanup old files and copy frontend build to static content.{{NORMAL}}'
  shopt -s nullglob
  files=(./axum-server/static-content/*)
  echo "cleaning up old files: ${files[@]}"
  [[ -e "${files[0]}" ]] && rm -r "${files[@]}"
  cp -r axum-server/measurements-frontend/dist/. axum-server/static-content/
  echo "fresh build staged under: ./axum-server/static-content/"

# format code for rp2350
[group: 'format']
fmt-pico:
  cargo fmt

# format code for the server
[group: 'format']
fmt-server:
  cargo fmt --manifest-path ./axum-server/Cargo.toml

# run all tests
[group: 'test']
test-all:
  cargo test --target=x86_64-unknown-linux-gnu -p tests -- --nocapture

# TEST := die|game|player
[group: 'test']
test TEST:
  cargo test --target=x86_64-unknown-linux-gnu -p tests --test test-{{TEST}} -- --nocapture

# build code for rp2350
[group: 'build']
build-all-pico:
  cargo build --all --features temperature

# build for rp2350; no temperature feature
[group: 'build']
build-all-pico-no-temperature:
  cargo build --all

# lint code for rp2350
[group: 'lint']
clippy-all-pico:
  cargo clippy --all --features temperature

# lint code for rp2350; no temperature feature
[group: 'lint']
clippy-all-pico-no-temperature:
  cargo clippy --all

# build the server
[group: 'build']
build-server:
  cargo build --manifest-path ./axum-server/Cargo.toml --target=x86_64-unknown-linux-gnu

# build the server podman image
[group: 'build']
build-server-image: stage-frontend
  podman compose -f ./axum-server/docker-compose.yaml build

# lint the server
[group: 'lint']
clippy-server:
  cargo clippy --manifest-path ./axum-server/Cargo.toml --target=x86_64-unknown-linux-gnu

# deploy and run the code on pico; debug probe required
[group: 'run']
run-pico:
  cargo run --release --features temperature

# the same as run-pico but no temperature feature
[group: 'run']
run-pico-no-temperature:
  cargo run --release

# run the server in podman
[group: 'run']
run-server: build-server-image
  podman compose -f ./axum-server/docker-compose.yaml up --force-recreate -d

# use before git push
[group: 'check']
pre-push: \
  fmt-pico \
  build-all-pico \
  build-all-pico-no-temperature \
  clippy-all-pico \
  clippy-all-pico-no-temperature \
  fmt-server \
  build-server \
  clippy-server \
  (frontend 'install-deps') \
  (frontend 'format') \
  (frontend 'lint') \
  (frontend 'build')
