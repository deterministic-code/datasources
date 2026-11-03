FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY samples/typescript/contacts-simple-cli /opt/src
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "-ec", "sh -e sqlite.sh && sh -e mysql-postgres.sh"]
