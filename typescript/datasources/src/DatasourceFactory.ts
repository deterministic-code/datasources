import type { IConnection } from "./IConnection.ts";
import type { IDatasource } from "./IDatasource.ts";
import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import { Datasource } from "./Datasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DotEnv } from "./dotEnv.ts";
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

const nonEmpty = (value: string | undefined): string | undefined =>
  value?.trim() ? value.trim() : undefined;

const merge = (file: DatasourceOptions, env: Record<string, string | undefined>): DatasourceOptions => {
  const options: DatasourceOptions = { ...file };
  for (const key of ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA", "PROVIDER", "provider"] as const) {
    const value = nonEmpty(env[key]);
    if (value) options[key] = value;
  }
  const url = nonEmpty(env.DEFAULT_DATASOURCE_URL) ?? get(options, "DATASOURCE_URL");
  const provider = nonEmpty(env.DEFAULT_PROVIDER) ?? get(options, "PROVIDER") ?? get(options, "provider");
  if (url) {
    options.DATASOURCE_URL = url;
    options.database_url = url;
  }
  if (provider) options.provider = provider;
  const schema = get(options, "DATABASE_SCHEMA");
  if (schema) options.database_schema = schema;
  return options;
};

const registerInstalled = (options: DatasourceOptions): void => install?.(options);

const resolve = (options?: DatasourceOptions): IDatasource => {
  const listed = options ?? merge(DotEnv.load(), process.env);
  if (options === undefined) registerInstalled(listed);
  const name = get(listed, "provider") ?? providerFromUrl(get(listed, "database_url") ?? "")
    ?? (() => { throw new DatasourceError("Datasource defaults are not specified"); })();
  const pair = factories.get(name);
  if (!pair) throw new ProviderNotInstalledError(name);
  return new Datasource(name, () => pair.connection(listed), pair.repository);
};

export class DatasourceFactory implements IDatasourceFactory {
  static get default(): IDatasource {
    return resolve();
  }

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
