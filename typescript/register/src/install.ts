import {
  bindInstall,
  DatasourceFactory,
  ProviderNotInstalledError,
  type DatasourceOptions,
  type IConnection,
} from "@datasources/core";
import { JsonConnection } from "@datasources/json";
import { MemoryConnection } from "@datasources/memory";
import { packages } from "./packages.ts";
import { providerNames } from "./providerNames.ts";

type Pair = {
  from: (options?: DatasourceOptions) => IConnection;
  repository: (connection: IConnection) => object;
};

const listed: Record<string, Pair> = {
  json: JsonConnection,
  memory: MemoryConnection,
};

const load = (name: string): Pair => {
  const pair = listed[name];
  if (!pair || !packages[name]) throw new ProviderNotInstalledError(name);
  return pair;
};

export const registerListed = (options: DatasourceOptions): void => {
  for (const name of providerNames(options)) {
    const pair = load(name);
    DatasourceFactory.register(name, pair.from, pair.repository);
  }
};

export const from = (options: DatasourceOptions): void => registerListed(options);

bindInstall(registerListed);
