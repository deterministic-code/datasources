FROM rust:1-alpine
RUN apk add --no-cache nodejs npm git curl build-base python3 pkgconf openssl-dev openssl-libs-static mariadb-client postgresql-client util-linux \
    && npm install -g @deterministic-code/datasources-cli
ENV CARGO_HOME=/cargo
ENV npm_config_cache=/npm-cache
WORKDIR /work
