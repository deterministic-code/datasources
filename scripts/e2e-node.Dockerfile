FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client util-linux
WORKDIR /work
COPY e2e-workdir.sh /e2e-workdir.sh
COPY e2e-quickstarts.sh /e2e-quickstarts.sh
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh --local && sh -e mysql-postgres.sh --local"]
