FROM rust:1-alpine
RUN apk add --no-cache nodejs npm git curl build-base python3 pkgconf openssl-dev openssl-libs-static mariadb-client postgresql-client
COPY e2e-sccache-install.sh /e2e-sccache-install.sh
RUN sh /e2e-sccache-install.sh
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV CARGO_HOME=/cargo
ENV npm_config_cache=/npm-cache
WORKDIR /work
COPY e2e-workdir.sh /e2e-workdir.sh
COPY e2e-quickstarts.sh /e2e-quickstarts.sh
COPY e2e-wait-engines.sh /e2e-wait-engines.sh
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"]
