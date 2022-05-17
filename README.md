# Clean architecture: rust

A Clean Architecture template for a Rest API in rust

# How it works

Motivations, explanations, requirements & more details in my article [Practical Clean Architecture in Typescript, Rust & Python](https://dev.to/msc29/practical-clean-architecture-in-typescript-rust-python-3a6d)

# Installing

```bash
cargo build
```

# Database setup

It's currently configured to run with PostgreSQL through Diesel (ORM), but this being clean architecture feel free to change it :)

I suggest

- PostgreSQL [in docker](https://hub.docker.com/_/postgres/)
- pgAdmin [install](https://www.pgadmin.org/download/pgadmin-4-apt/)

Then install [diesel_cli](https://lib.rs/crates/diesel_cli)

```bash
diesel setup --database-url postgresql://postgres:postgres@localhost/animal_fact_db
diesel migration run --database-url postgresql://postgres:postgres@localhost/animal_fact_db
```

# Running

define the environment on which we're running by adding `ENV=<env>`, which will use the `.env.<env>` file

```bash
ENV=dev cargo run
```

# Code quality & security

Used in CI/CD

```bash
cargo fmt --all -- --check
cargo clippy --all-targets
cargo audit
cargo outdated
```

# Testing

Here's what done in order to mock the SPI

- db: every test is creating a new database from `TestContextPostgreSQL` with json fixtures in `test/fixtures` & spawns the app with this database
- http: every test also spins up another rust api (if not already up) with the expected routes but test data in `test/fixtures`

```bash
ENV=test cargo test
```

# API Documentation

TODO: https://github.com/paperclip-rs/paperclip
