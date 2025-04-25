watch:
    cargo watch -x run -w src

fmt:
  cargo fmt --all
  cargo clippy --all-features -- -D warnings
