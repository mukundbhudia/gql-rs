# gql-rs

[![Rust](https://github.com/mukundbhudia/gql-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mukundbhudia/gql-rs/actions/workflows/rust.yml)

A basic GraphQL server for Rust.

Specs TBC...

## Goals
- Learn the following by building with:
  - The `tide` crate
  - The `async-std` crate
  - The `async-graphql` crate & creating mutations and queries
  - Diesel ORM & recap on relational databases
  - `tracing` crate

## Prerequisites
- Rust
- Docker
- Docker Compose

## Database setup

Refer to https://diesel.rs/guides/getting-started/ for more information.

- Run `cargo install diesel_cli`
- Run `make db-start` which is basically `docker-compose up -d`
- Run `diesel setup`
- The database should now be ready to connect to.

## Development

- `make run` and then http://localhost:5050/graphiql (which should be pointing to http://localhost:5050/query)
- Enter queries and mutations in the GraphiQL interface and see the results in the console.
