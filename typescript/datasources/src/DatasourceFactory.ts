import type { IConnection } from "./IConnection.ts";
import type { IDatasource } from "./IDatasource.ts";
import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import { Datasource } from "./Datasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";
import { providerFromUrl } from "./providerFromUrl.ts";

type Pair = {
  connection: (options?: DatasourceOptions) => IConnection;
  repository: (connection: IConnection) => object;
};

const factories = new Map<string, Pair>();

type Install = (options: DatasourceOptions) => void;
let install: Install | undefined;

export const bindInstall = (fn?: Install): void => {
  install = fn;
};

const get = (options: DatasourceOptions | undefined, key: string): string | undefined => {
  const value = options?.[key];
  return value && value.trim() ? value.trim() : undefined;
};

const connectionFromEnvironment = (): DatasourceOptions => {
  const options: DatasourceOptions = {};
  const url = process.env.DATABASE_URL;
  if (url?.trim()) options.database_url = url;
  const schema = process.env.DATABASE_SCHEMA;
  if (schema?.trim()) options.database_schema = schema;
  return options;
};

const installFromEnvironment = (): DatasourceOptions => {
  const options: DatasourceOptions = {};
  for (const key of ["INSTALLED_PROVIDERS", "DATABASE_URL"] as const) {
    const value = process.env[key];
    if (value?.trim()) options[key] = value;
  }
  return options;
};

const registerInstalled = (options: DatasourceOptions): void => install?.(options);

const resolve = (options?: DatasourceOptions): IDatasource => {
  if (options === undefined) registerInstalled(installFromEnvironment());
  const listed = options ?? connectionFromEnvironment();
  const name = get(listed, "provider") ?? providerFromUrl(get(listed, "database_url") ?? "")
    ?? (() => { throw new DatasourceError("Datasource defaults are not specified"); })();
  const pair = factories.get(name);
  if (!pair) throw new ProviderNotInstalledError(name);
  return new Datasource(name, () => pair.connection(listed), pair.repository);
};

export class DatasourceFactory implements IDatasourceFactory {
  static defaults = (options: DatasourceOptions): void => registerInstalled(options);

  static register = (
    provider: string,
    connection: (options?: DatasourceOptions) => IConnection,
    repository: (connection: IConnection) => object,
  ): void => {
    factories.set(provider, { connection, repository });
  };

  static create = (options?: DatasourceOptions): IDatasource => resolve(options);

  create = (options?: DatasourceOptions): IDatasource => resolve(options);

  static reset = (): void => {
    factories.clear();
  };

  static registered = (): string[] => [...factories.keys()].sort();
}
