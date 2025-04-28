run:
    cargo run
watch:
    cargo watch -x run -w src

fmt:
  cargo fmt --all
  cargo clippy --all-features -- -D warnings

log:
    tail -n 10 -f terraformed.log
