export const providers = [
  "json",
  "memory",
  "sqlite",
  "xml",
  "yaml",
  "mysql",
  "sqlserver",
  "postgres",
  "oracle",
  "mongodb",
  "dynamodb",
] as const;

export type Provider = (typeof providers)[number];

export type Ecosystem = "ts" | "rust" | "csharp";

export type ProviderPackages = {
  npm: string;
  cargo: string;
  nuget: string;
};

export const catalog: Record<Provider, ProviderPackages> = {
  json: { npm: "@datasources/json", cargo: "datasources-json", nuget: "Datasources.Json" },
  memory: { npm: "@datasources/memory", cargo: "datasources-memory", nuget: "Datasources.Memory" },
  sqlite: { npm: "@datasources/sqlite", cargo: "datasources-sqlite", nuget: "Datasources.Sqlite" },
  xml: { npm: "@datasources/xml", cargo: "datasources-xml", nuget: "Datasources.Xml" },
  yaml: { npm: "@datasources/yaml", cargo: "datasources-yaml", nuget: "Datasources.Yaml" },
  mysql: {
    npm: "@datasources/mysql",
    cargo: "datasources-mysql",
    nuget: "Datasources.Mysql",
  },
  sqlserver: {
    npm: "@datasources/sqlserver",
    cargo: "datasources-sqlserver",
    nuget: "Datasources.SqlServer",
  },
  postgres: {
    npm: "@datasources/postgres",
    cargo: "datasources-postgres",
    nuget: "Datasources.Postgres",
  },
  oracle: { npm: "@datasources/oracle", cargo: "datasources-oracle", nuget: "Datasources.Oracle" },
  mongodb: { npm: "@datasources/mongodb", cargo: "datasources-mongodb", nuget: "Datasources.MongoDb" },
  dynamodb: {
    npm: "@datasources/dynamodb",
    cargo: "datasources-dynamodb",
    nuget: "Datasources.DynamoDb",
  },
};

export const host = {
  ts: { core: "@datasources/core" },
  rust: { core: "datasources" },
  csharp: { core: "Datasources" },
} as const;

export const isProvider = (name: string): name is Provider =>
  (providers as readonly string[]).includes(name);

export const requireProviders = (names: string[]): Provider[] =>
  names.map((name) =>
    isProvider(name)
      ? name
      : (() => {
          throw new Error(`Unknown provider: ${name}. Choose from: ${providers.join(", ")}`);
        })(),
  );
