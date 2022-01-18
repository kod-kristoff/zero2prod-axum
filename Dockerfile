FROM rust:1.57

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]
