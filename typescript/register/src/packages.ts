export const packages: Record<string, string> = {
  json: "@datasources/json",
  memory: "@datasources/memory",
  sqlite: "@datasources/sqlite",
  text: "@datasources/text",
  xml: "@datasources/xml",
  yaml: "@datasources/yaml",
  mysql: "@datasources/mysql",
  sqlserver: "@datasources/sqlserver",
  postgres: "@datasources/postgres",
  oracle: "@datasources/oracle",
  mongodb: "@datasources/mongodb",
  dynamodb: "@datasources/dynamodb",
};

export const typeNames: Record<string, string> = {
  json: "JsonDatasource",
  memory: "MemoryDatasource",
  sqlite: "SqliteDatasource",
  text: "TextDatasource",
  xml: "XmlDatasource",
  yaml: "YamlDatasource",
  mysql: "MysqlDatasource",
  sqlserver: "SqlServerDatasource",
  postgres: "PostgresDatasource",
  oracle: "OracleDatasource",
  mongodb: "MongoDbDatasource",
  dynamodb: "DynamoDbDatasource",
};
