FROM rust:1.86 as builder
WORKDIR /usr/src/tyche-api

RUN cargo install sqlx-cli --version 0.8.3 --features postgres

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src src
COPY .sqlx .sqlx

RUN cargo build --release

FROM debian:12.10
WORKDIR /app
COPY --from=builder /usr/src/tyche-api/target/release/tyche .

EXPOSE 8000

CMD ["./tyche"]
