import {
  Datasource,
  ProviderNotInstalledError,
  type IDatasourceFactory,
  type DatasourceOptions,
} from "@datasources/core";
import { JsonDatasource } from "@datasources/json";
import { MemoryDatasource } from "@datasources/memory";
import { packages, typeNames } from "./packages.ts";
import { providerNames } from "./providerNames.ts";

export type ProviderLoader = (pkg: string) => Promise<object>;

type FactoryHolder = { readonly instance: IDatasourceFactory };

const cache = new Map<string, IDatasourceFactory>([
  ["json", JsonDatasource.instance],
  ["memory", MemoryDatasource.instance],
]);

const factoryOf = (mod: object, name: string): IDatasourceFactory => {
  const exported = (mod as Record<string, FactoryHolder | undefined>)[typeNames[name]];
  return exported?.instance ?? (() => { throw new ProviderNotInstalledError(name); })();
};

const defaultLoader: ProviderLoader = (pkg) => import(pkg);

const load = (name: string): IDatasourceFactory => {
  const factory = cache.get(name);
  return factory ?? (() => { throw new ProviderNotInstalledError(name); })();
};

const registerListed = (options: DatasourceOptions): void => {
  for (const name of providerNames(options)) Datasource.register(load(name));
};

Datasource.bindInstall(registerListed);

export const install = async (
  options: DatasourceOptions,
  loader: ProviderLoader = defaultLoader,
): Promise<void> => {
  for (const name of providerNames(options)) {
    const pkg = packages[name];
    if (!pkg) throw new ProviderNotInstalledError(name);
    try {
      const factory = factoryOf(await loader(pkg), name);
      cache.set(name, factory);
      Datasource.register(factory);
    } catch (error) {
      throw error instanceof ProviderNotInstalledError
        ? error
        : new ProviderNotInstalledError(name);
    }
  }
  Datasource.defaults(options);
};
