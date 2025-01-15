default:
    just --list

fmt:
    cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt
