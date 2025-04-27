# Tyche API (Backend)

## Prerequisites

The following tools are required on your local machine before Tyche can start
running:

1. [The Rust Toolchain](https://www.rust-lang.org/tools/install)
2. [Docker](https://docs.docker.com/get-started/get-docker/)
3. [direnv](https://direnv.net/docs/installation.html)
4. [pre-commit](https://pre-commit.com/#intro)
5. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install)
6. [GNU Make](https://www.gnu.org/software/make/)

When prompted, run the following from the root of the repository:

```sh
direnv allow
```

## Environment Variables

The following environment variables must be defined in an `.env` file at the
root of the repository:

| Variable Key             | Variable Description                                                           |
| ------------------------ | ------------------------------------------------------------------------------ |
| `ENVIRONMENT_TYPE`       | The type of environment you're working in (development, production, etc.)      |
| `ALLOWED_ORIGIN`         | The allowed origin.                                                            |
| `DATABASE_USER_FILE`     | An absolute path to a file containing the DB username.                         |
| `DATABASE_PASSWORD_FILE` | An absolute path to a file containing the DB password.                         |
| `DATABASE_URL_FILE`      | An absolute path to a file containing the DB URL.                              |
| `JWT_SECRET_FILE`        | An absolute path to a file containing a secret that will be used to sign JWTs. |

The above environment variables should not contain sensitive information. The
files some variables point to may contain sensitive information.

## Running

First, run the following from the root of the repository to start a development
PostgreSQL server in a docker container:

```sh
make db-create
```

Next, run the following to execute the migration scripts for the database:

```sh
make db-migrate-run
```

Finally, run the following to start the development build of the application:

```sh
cargo run
```
