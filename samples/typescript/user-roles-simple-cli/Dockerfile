FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY . /work
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
