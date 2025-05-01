# List all available recipes
default:
    @just --list

# Run with info log level
run:
    cargo run -- --log-level=info

# Run with debug log level
debug:
    cargo run -- --log-level=debug

# Listen for changes and run
watch:
    cargo watch -w src -s 'clear && cargo run'

# Run fmt and clippy
fmt:
  cargo fmt --all
  cargo clippy --all-features -- -D warnings

# Tail the log file
log:
    tail -n 10 -f terraformed.log
