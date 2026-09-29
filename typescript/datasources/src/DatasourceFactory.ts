import type { IDatasource } from "./IDatasource.ts";
import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { IDriver } from "./IDriver.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import { Datasource } from "./Datasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DatasourceUrlParser } from "./DatasourceUrlParser.ts";
import { DriverLoader } from "./DriverLoader.ts";
import { DotEnv } from "./dotEnv.ts";
import { ProviderCatalog } from "./ProviderCatalog.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

const factories = new Map<string, IDriver>();

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

const split = (value: string | undefined): string[] =>
  (value ?? "")
    .split(",")
    .map((name) => name.trim())
    .filter((name) => name.length > 0);

function* installed(options: DatasourceOptions): Generator<string> {
  const names = new Set<string>();
  for (const name of split(get(options, "INSTALLED_PROVIDERS"))) {
    if (names.has(name)) continue;
    names.add(name);
    yield name;
  }
  const url = get(options, "DATASOURCE_URL");
  if (!url) return;
  const provider = DatasourceUrlParser.parse(url)?.provider;
  if (!provider) throw new DatasourceError("DATASOURCE_URL has no provider");
  if (!names.has(provider)) yield provider;
}

const registerInstalled = (options: DatasourceOptions): void => {
  for (const name of installed(options)) {
    const catalog = ProviderCatalog.default[name];
    if (!catalog) throw new ProviderNotInstalledError(name);
    DatasourceFactory.register(name, DriverLoader.load(catalog));
  }
};

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
