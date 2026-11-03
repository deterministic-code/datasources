FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine
RUN apk add --no-cache nodejs npm git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY samples/csharp/contacts-simple-cli /opt/src
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
