export MEASUREMENTS_ENDPOINT := "http://192.168.132.170:5000/api/measurements"

set dotenv-required
set dotenv-path := "hub.env"

# check for outdated dependencies
outdated:
  # run `cargo upgrade --dry-run` to check versions defined in [workspace.dependencies]
  cargo outdated -w --root-deps-only

# build the server
build-server:
  cargo build --manifest-path ./axum-server/Cargo.toml --target=x86_64-unknown-linux-gnu

# build the server podman image
build-server-image:
  podman compose -f ./axum-server/docker-compose.yaml build

# buld code for rp2350
build-all-pico:
  cargo build --all

# TEST := die|game|player
test TEST:
  cargo test --target=x86_64-unknown-linux-gnu -p tests --test test-{{TEST}} -- --nocapture

# use before git push
pre-push: build-all-pico build-server
