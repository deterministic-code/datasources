FROM rust:1-alpine
RUN apk add --no-cache nodejs npm git build-base python3 pkgconf openssl-dev openssl-libs-static mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY scripts/e2e-quickstarts.sh /e2e-quickstarts.sh
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "/e2e-quickstarts.sh"]
