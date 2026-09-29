import type { IDatasource } from "./IDatasource.ts";
import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { IDriver } from "./IDriver.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import { Datasource } from "./Datasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DotEnv } from "./dotEnv.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";
const factories = new Map<string, IDriver>();

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
  for (const key of ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"] as const) {
    const value = nonEmpty(env[key]);
    if (value) options[key] = value;
  }
  return options;
};

const registerInstalled = (options: DatasourceOptions): void => install?.(options);

const resolve = (options?: DatasourceOptions): IDatasource => {
  const listed = options ?? merge(DotEnv.load(), process.env);
  if (options === undefined) registerInstalled(listed);
  const name = get(listed, "PROVIDER")
    ?? (() => { throw new DatasourceError("Datasource defaults are not specified"); })();
  const driver = factories.get(name);
  if (!driver) throw new ProviderNotInstalledError(name);
  return new Datasource(name, driver, listed);
};

export class DatasourceFactory implements IDatasourceFactory {
  static get default(): IDatasource {
    return resolve();
  }

  static defaults = (options: DatasourceOptions): void => registerInstalled(options);

  static register = (provider: string, driver: IDriver): void => {
    factories.set(provider, driver);
  };

  static create = (options?: DatasourceOptions): IDatasource => resolve(options);

  create = (options?: DatasourceOptions): IDatasource => resolve(options);

  static reset = (): void => {
    factories.clear();
  };

  static registered = (): string[] => [...factories.keys()].sort();
}
