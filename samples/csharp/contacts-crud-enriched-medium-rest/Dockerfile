FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine
RUN apk add --no-cache nodejs npm git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY rust /opt/repo/rust
COPY typescript /opt/repo/typescript
COPY csharp /opt/repo/csharp
COPY cli /opt/repo/cli
COPY samples/csharp/contacts-crud-enriched-medium-rest /opt/src
ENV SAMPLE=samples/csharp/contacts-crud-enriched-medium-rest
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"]
