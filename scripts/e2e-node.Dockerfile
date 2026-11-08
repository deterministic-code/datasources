FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client util-linux \
    && npm install -g @deterministic-code/datasources-cli
WORKDIR /work
