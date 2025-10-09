FROM node:26.1.0-trixie-slim AS node
FROM rust:1.95.0-slim-trixie
COPY --from=node /usr/local/lib/ /usr/local/lib/
COPY --from=node /usr/local/bin/ /usr/local/bin/
RUN apt update && apt install --no-install-recommends -y procps
RUN cargo install sqlx-cli --version 0.8.6 -F postgres,rustls --no-default-features
RUN cargo install watchexec-cli --version 2.5.1 --no-default-features
