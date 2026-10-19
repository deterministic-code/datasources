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
  json: { npm: "@deterministic-code/datasources-json", cargo: "datasources-json", nuget: "Datasources.Json" },
  memory: { npm: "@deterministic-code/datasources-memory", cargo: "datasources-memory", nuget: "Datasources.Memory" },
  sqlite: { npm: "@deterministic-code/datasources-sqlite", cargo: "datasources-sqlite", nuget: "Datasources.Sqlite" },
  xml: { npm: "@deterministic-code/datasources-xml", cargo: "datasources-xml", nuget: "Datasources.Xml" },
  yaml: { npm: "@deterministic-code/datasources-yaml", cargo: "datasources-yaml", nuget: "Datasources.Yaml" },
  mysql: {
    npm: "@deterministic-code/datasources-mysql",
    cargo: "datasources-mysql",
    nuget: "Datasources.Mysql",
  },
  sqlserver: {
    npm: "@deterministic-code/datasources-sqlserver",
    cargo: "datasources-sqlserver",
    nuget: "Datasources.SqlServer",
  },
  postgres: {
    npm: "@deterministic-code/datasources-postgres",
    cargo: "datasources-postgres",
    nuget: "Datasources.Postgres",
  },
  oracle: { npm: "@deterministic-code/datasources-oracle", cargo: "datasources-oracle", nuget: "Datasources.Oracle" },
  mongodb: { npm: "@deterministic-code/datasources-mongodb", cargo: "datasources-mongodb", nuget: "Datasources.MongoDb" },
  dynamodb: {
    npm: "@deterministic-code/datasources-dynamodb",
    cargo: "datasources-dynamodb",
    nuget: "Datasources.DynamoDb",
  },
};

export const host = {
  ts: { core: "@deterministic-code/datasources" },
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
