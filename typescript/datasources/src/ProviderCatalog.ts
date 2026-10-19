export class ProviderCatalog {
  constructor(
    readonly name: string,
    readonly location: string,
    readonly module: string,
  ) {}

  static readonly default: Readonly<Record<string, ProviderCatalog>> = {
    json: new ProviderCatalog("json", "@deterministic-code/datasources-json", "JsonDriver"),
    memory: new ProviderCatalog("memory", "@deterministic-code/datasources-memory", "MemoryDriver"),
    sqlite: new ProviderCatalog("sqlite", "@deterministic-code/datasources-sqlite", "SqliteDriver"),
    xml: new ProviderCatalog("xml", "@deterministic-code/datasources-xml", "XmlDriver"),
    yaml: new ProviderCatalog("yaml", "@deterministic-code/datasources-yaml", "YamlDriver"),
    mysql: new ProviderCatalog("mysql", "@deterministic-code/datasources-mysql", "MysqlDriver"),
    sqlserver: new ProviderCatalog("sqlserver", "@deterministic-code/datasources-sqlserver", "SqlServerDriver"),
    postgres: new ProviderCatalog("postgres", "@deterministic-code/datasources-postgres", "PostgresDriver"),
    oracle: new ProviderCatalog("oracle", "@deterministic-code/datasources-oracle", "OracleDriver"),
    mongodb: new ProviderCatalog("mongodb", "@deterministic-code/datasources-mongodb", "MongoDbDriver"),
    dynamodb: new ProviderCatalog("dynamodb", "@deterministic-code/datasources-dynamodb", "DynamoDbDriver"),
  };
}
