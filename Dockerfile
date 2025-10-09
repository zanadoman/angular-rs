FROM node:25-slim
FROM rustlang/rust:nightly-slim
COPY --from=node:25-slim /usr/local/lib/ /usr/local/lib/
COPY --from=node:25-slim /usr/local/bin/ /usr/local/bin/
RUN apt update && apt install -y --no-install-recommends chromium
RUN rustup component add rust-docs rustfmt clippy
RUN cargo install sqlx-cli --no-default-features -F mysql,rustls
RUN cargo install watchexec-cli --no-default-features
WORKDIR /app
