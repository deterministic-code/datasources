import {
  bindInstall,
  DatasourceFactory,
  ProviderNotInstalledError,
  type DatasourceOptions,
  type IDriver,
} from "@datasources/core";
import { JsonDriver } from "@datasources/json";
import { MemoryDriver } from "@datasources/memory";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import { packages } from "./packages.ts";
import { providerNames } from "./providerNames.ts";

const listed: Record<string, new () => IDriver> = {
  json: JsonDriver,
  memory: MemoryDriver,
  sqlite: SqliteDriver,
  mysql: MysqlDriver,
  postgres: PostgresDriver,
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
