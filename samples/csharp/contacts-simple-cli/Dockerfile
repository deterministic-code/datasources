FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine
RUN apk add --no-cache nodejs npm git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY . /opt/src
ENTRYPOINT ["sh", "-euc", "find /work -mindepth 1 -delete; if [ -e /repo/typescript/datasources/package.json ]; then cp -a /repo/. /work; else cp -a /opt/src/. /work; fi; cd \"${E2E_WORKDIR:-/work}\"; exec \"$@\"", "sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
