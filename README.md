# Clean architecture: rust

A Clean Archtiecture template for a Rest API in rust

# How it works

TODO: proper doc & link to Rust & TypeScript repo

TODO: documenting key interfaces & classes for clearer understanding & to evidence benefits of Clean Architecture

# Installing

```bash
cargo build
```

# Database setup

it's currently configued to run with Postgresl through Diesel (ORM), but this being clean architecture feel free to change it :)

I suggest

- postgresql [in docker](https://hub.docker.com/_/postgres/)
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
fmt --all -- --check
clippy --all-targets
cargo audit
```

# Testing

```bash
ENV=test pytest
```

# API Documentation

`http://127.0.0.1:8888/docs`
