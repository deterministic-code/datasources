import {
  bindInstall,
  DatasourceFactory,
  ProviderNotInstalledError,
  type DatasourceOptions,
  type IDriver,
} from "@datasources/core";
import { JsonProvider } from "@datasources/json";
import { MemoryProvider } from "@datasources/memory";
import { MysqlProvider } from "@datasources/mysql";
import { PostgresProvider } from "@datasources/postgres";
import { SqliteProvider } from "@datasources/sqlite";
import { packages } from "./packages.ts";
import { providerNames } from "./providerNames.ts";

const listed: Record<string, new () => IDriver> = {
  json: JsonProvider,
  memory: MemoryProvider,
  sqlite: SqliteProvider,
  mysql: MysqlProvider,
  postgres: PostgresProvider,
};

const load = (name: string): IDriver => {
  const Type = listed[name];
  if (!Type || !packages[name]) throw new ProviderNotInstalledError(name);
  return new Type();
};

export const registerListed = (options: DatasourceOptions): void => {
  for (const name of providerNames(options))
    DatasourceFactory.register(name, load(name));
};

export const from = (options: DatasourceOptions): void => registerListed(options);

bindInstall(registerListed);
