FROM rust:1.57 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build the application
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

FROM debian:bullseye-slim AS runtime 

WORKDIR /app

RUN apt-get update -y \ 
    && apt-get install -y --no-install-recommends openssl \ 
    # Clean up 
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

