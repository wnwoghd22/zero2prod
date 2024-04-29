# latest stable release
FROM rust:1.77.2

# switch working directory
WORKDIR /app

# install the required system dependencies
RUN apt update && apt install lld clang -y

# copy all files from working env to docker img
COPY . .

ENV SQLX_OFFLINE true

# build
RUN cargo build --release

# entry point
ENTRYPOINT ["./target/release/zero2prod"]
