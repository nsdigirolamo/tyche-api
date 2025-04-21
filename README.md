# Tyche

## Prerequisites

The following tools are required on your local machine before Tyche can start
running:

1. [The Rust Toolchain](https://www.rust-lang.org/tools/install)
2. [Docker](https://docs.docker.com/get-started/get-docker/)
3. [direnv](https://direnv.net/docs/installation.html)
4. [pre-commit](https://pre-commit.com/#intro)
5. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install)

When prompted, run `direnv allow` from the root of Tyche's repository.

## Environment Variables

Tyche requires the following environment variables be defined:

| Variable Key        | Variable Description                         |
| ------------------- | -------------------------------------------- |
| `DATABASE_URL`      | The URL to an available PostgreSQL database. |
| `DATABASE_PASSWORD` | The password to the above database.          |
| `JWT_SECRET`        | The secret Tyche will use to sign its JWTs.  |
