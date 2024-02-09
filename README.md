# `rust_decimal` Postgres Deserialization Issue

This repository is a minimal reproduction of a deserialization issue with the `rust_decimal` crate's `db-postgres` feature when used with the `tokio-postgres` crate.

## Reproduction Steps

1. Clone this repository
2. Run `docker compose up` to start a DB with some pre-seeded data
3. Run `cargo run` to run the example code
4. Observe the output to see what goes wrong