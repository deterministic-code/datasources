FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine
# Prefix install extracts dependencies. npm install -g leaves them empty when bundledDependencies is set.
RUN apk add --no-cache nodejs npm git build-base python3 mariadb-client postgresql-client util-linux curl \
    && apk add --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/v3.22/community mono \
    && mkdir -p /opt/datasources-cli \
    && npm install --prefix /opt/datasources-cli @deterministic-code/datasources-cli \
    && printf '%s\n' '#!/bin/sh' 'exec node /opt/datasources-cli/node_modules/@deterministic-code/datasources-cli/bin/datasources.js "$@"' > /usr/local/bin/datasources \
    && chmod +x /usr/local/bin/datasources
RUN curl -fsSL https://dist.nuget.org/win-x86-commandline/v6.14.0/nuget.exe -o /usr/local/lib/nuget.exe \
    && printf '#!/bin/sh\nexec mono /usr/local/lib/nuget.exe "$@"\n' > /usr/local/bin/nuget \
    && chmod +x /usr/local/bin/nuget
WORKDIR /work
