set shell := ["bash", "-cu"]

default:
    cargo check

dev:
    cargo run

test:
    cargo test

lint:
    cargo fmt -- --check
    cargo clippy -- -D warnings

fix:
    cargo fmt
    cargo clippy --fix --allow-dirty

release:
    cargo build --release

coverage:
    cargo tarpaulin --ignore-tests

audit:
    cargo audit

docker-build:
    docker build -t hello_trunk .

docker-run:
    docker run --rm hello_trunk