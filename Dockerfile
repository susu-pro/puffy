FROM rust:1.83-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p puffy-cli

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/puffy /usr/local/bin/puffy
COPY docker/entrypoint.sh /entrypoint.sh
ENTRYPOINT ["sh", "/entrypoint.sh"]

