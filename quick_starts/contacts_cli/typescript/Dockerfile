FROM node:22-alpine
RUN apk add --no-cache git build-base python3 mariadb-client postgresql-client
WORKDIR /work
COPY scripts/e2e-workdir.sh /e2e-workdir.sh
COPY scripts/e2e-quickstarts.sh /e2e-quickstarts.sh
ENTRYPOINT ["sh", "/e2e-workdir.sh"]
CMD ["sh", "/e2e-quickstarts.sh"]
