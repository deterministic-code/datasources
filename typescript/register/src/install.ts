import {
  Datasource,
  ProviderNotInstalledError,
  type DatasourceFactory,
  type DatasourceOptions,
} from "@datasources/core";
import { packages, typeNames } from "./packages.ts";
import { providerNames } from "./providerNames.ts";

export type ProviderLoader = (pkg: string) => Promise<object>;

type FactoryHolder = { readonly factory: DatasourceFactory };

const factoryOf = (mod: object, name: string): DatasourceFactory => {
  const exported = (mod as Record<string, FactoryHolder | undefined>)[typeNames[name]];
  return exported?.factory ?? (() => { throw new ProviderNotInstalledError(name); })();
};

const defaultLoader: ProviderLoader = (pkg) => import(pkg);

export const install = async (
  options: DatasourceOptions,
  loader: ProviderLoader = defaultLoader,
): Promise<void> => {
  for (const name of providerNames(options)) {
    const pkg = packages[name];
    if (!pkg) throw new ProviderNotInstalledError(name);
    try {
      Datasource.use(factoryOf(await loader(pkg), name));
    } catch (error) {
      throw error instanceof ProviderNotInstalledError
        ? error
        : new ProviderNotInstalledError(name);
    }
  }
  Datasource.defaults(options);
};
