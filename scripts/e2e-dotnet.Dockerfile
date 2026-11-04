FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine
RUN apk add --no-cache nodejs npm git build-base python3 mariadb-client postgresql-client util-linux curl \
    && apk add --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/v3.22/community mono
RUN curl -fsSL https://dist.nuget.org/win-x86-commandline/v6.14.0/nuget.exe -o /usr/local/lib/nuget.exe \
    && printf '#!/bin/sh\nexec mono /usr/local/lib/nuget.exe "$@"\n' > /usr/local/bin/nuget \
    && chmod +x /usr/local/bin/nuget
WORKDIR /work
COPY e2e-workdir.sh /e2e-workdir.sh
COPY e2e-quickstarts.sh /e2e-quickstarts.sh
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"]
