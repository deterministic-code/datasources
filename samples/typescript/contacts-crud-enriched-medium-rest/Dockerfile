FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY rust /opt/repo/rust
COPY typescript /opt/repo/typescript
COPY csharp /opt/repo/csharp
COPY cli /opt/repo/cli
COPY samples/typescript/contacts-crud-enriched-medium-rest /opt/src
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
