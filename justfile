export RUSTFLAGS := '-C target-cpu=native'

dev:
    cargo run

update:
    cargo update

clippy:
    cargo clippy --workspace --lib --examples --tests --benches --all-features --locked --fix --allow-dirty --allow-staged

build:
    cargo build

pre: fmt test

test:
    cd {{invocation_directory()}}; cargo +nightly nextest run --all-features

testt:
    cargo +nightly nextest run --all-features --workspace

fmt:
    cd {{invocation_directory()}}; cargo +nightly fmt ; cargo +nightly clippy --examples --tests --benches --all-features --fix --allow-dirty  --allow-staged

fmtt:
    cargo +nightly fmt --all ; cargo +nightly clippy --workspace --lib --examples --tests --benches --all-features --fix --allow-dirty  --allow-staged
