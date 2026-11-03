FROM rust:1-alpine
RUN apk add --no-cache nodejs npm git build-base python3 pkgconf openssl-dev openssl-libs-static mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY rust /opt/repo/rust
COPY typescript /opt/repo/typescript
COPY csharp /opt/repo/csharp
COPY cli /opt/repo/cli
COPY samples/rust/contacts-crud-enriched-medium-rest /opt/src
ENV SAMPLE=samples/rust/contacts-crud-enriched-medium-rest
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"]
