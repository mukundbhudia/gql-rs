# gql-rs

A basic GraphQL server for Rust.

## Database setup

Refer to https://diesel.rs/guides/getting-started/ for more information.

- Run `cargo install diesel_cli`
- Run `make db-start` which is basically `docker-compose up -d`
- Run `diesel setup`
- The database should now be ready to connect to.
