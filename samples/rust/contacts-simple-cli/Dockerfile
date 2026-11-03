FROM rust:1-alpine
RUN apk add --no-cache nodejs npm git build-base python3 pkgconf openssl-dev mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY samples/rust/contacts-simple-cli /opt/src
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
