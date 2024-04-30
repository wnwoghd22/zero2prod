# Builder stage

# latest stable release
FROM rust:1.77.2 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM rust:1.77.2 AS runtime
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]
