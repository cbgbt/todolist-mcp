## Implementing Feature

- Whenever you implement a feature or create a code change you MAY opt to run all checks at once, with `make check`, or you:

- MUST run the build `cargo build -q`
- MUST run tests `cargo test -q`
- MUST run clippy `cargo clippy -q`
- MUST format code with `cargo fmt -q`
- MUST fix any issues uncovered with these commands
