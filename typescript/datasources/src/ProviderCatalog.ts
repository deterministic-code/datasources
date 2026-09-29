export class ProviderCatalog {
  constructor(
    readonly name: string,
    readonly location: string,
    readonly module: string,
  ) {}

  static readonly default: Readonly<Record<string, ProviderCatalog>> = {
    json: new ProviderCatalog("json", "@datasources/json", "JsonDriver"),
    memory: new ProviderCatalog("memory", "@datasources/memory", "MemoryDriver"),
    sqlite: new ProviderCatalog("sqlite", "@datasources/sqlite", "SqliteDriver"),
    xml: new ProviderCatalog("xml", "@datasources/xml", "XmlDriver"),
    yaml: new ProviderCatalog("yaml", "@datasources/yaml", "YamlDriver"),
    mysql: new ProviderCatalog("mysql", "@datasources/mysql", "MysqlDriver"),
    sqlserver: new ProviderCatalog("sqlserver", "@datasources/sqlserver", "SqlServerDriver"),
    postgres: new ProviderCatalog("postgres", "@datasources/postgres", "PostgresDriver"),
    oracle: new ProviderCatalog("oracle", "@datasources/oracle", "OracleDriver"),
    mongodb: new ProviderCatalog("mongodb", "@datasources/mongodb", "MongoDbDriver"),
    dynamodb: new ProviderCatalog("dynamodb", "@datasources/dynamodb", "DynamoDbDriver"),
  };
}
