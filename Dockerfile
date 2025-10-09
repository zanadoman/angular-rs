FROM node:25-slim
FROM rust:1.91-slim
COPY --from=node:25-slim /usr/local/lib/ /usr/local/lib/
COPY --from=node:25-slim /usr/local/bin/ /usr/local/bin/
RUN apt update && apt install -y --no-install-recommends chromium
RUN cargo install sqlx-cli --version 0.8.6 -F mysql,rustls --no-default-features
RUN cargo install watchexec-cli --version 2.3.2 --no-default-features
WORKDIR /app
