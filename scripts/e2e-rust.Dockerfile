FROM rust:1-alpine
# Prefix install extracts dependencies. npm install -g leaves them empty when bundledDependencies is set.
RUN apk add --no-cache nodejs npm git curl build-base python3 pkgconf openssl-dev openssl-libs-static mariadb-client postgresql-client util-linux \
    && mkdir -p /opt/datasources-cli \
    && npm install --prefix /opt/datasources-cli @deterministic-code/datasources-cli \
    && printf '%s\n' '#!/bin/sh' 'exec node /opt/datasources-cli/node_modules/@deterministic-code/datasources-cli/bin/datasources.js "$@"' > /usr/local/bin/datasources \
    && chmod +x /usr/local/bin/datasources
ENV CARGO_HOME=/cargo
ENV npm_config_cache=/npm-cache
WORKDIR /work
