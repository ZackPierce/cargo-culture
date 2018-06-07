# cargo-culture

Automated opinionated checks for Rust project compliance with conventions and intention towards excellence.

## How?

```bash
cargo install cargo-culture

cargo culture
```

## Why?

This tool simulates having an experienced Rustacean engineer do a quick sanity-check over your project.

The rules were developed with open-source collaboration and a safety-first keep-trying-hard attitude in mind.

`cargo-culture` is subjective. It's okay if you don't agree with all of its suggestions, just like you might not
want to take 100% of your mentor's nitpicks to heart.

## Current Default Rules

A good project...

* Should include a well-formed Cargo.toml readable by `cargo metadata`
* Should `cargo clean` and `cargo build` without warnings or errors.
* Should have a README.md file in the project directory.
* Should have a LICENSE file in the project directory.
* Should have a CONTRIBUTING file in the project directory.
* Should have a rustfmt.toml in the project directory.
* Should have a file suggesting the use of a continuous integration system.
* Should have multiple tests which pass.
* Should be making an effort to use property based tests.

## Future Rules

A great project...

* Should be measuring its test coverage.
* Should be fuzz-testing all of its binaries.
* Should contain some benchmarks.

