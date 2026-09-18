import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { IDatasource } from "./IDatasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";
import { providerFromUrl } from "./providerFromUrl.ts";

const registry = new Map<string, IDatasourceFactory>();
let specified: DatasourceOptions | undefined;

const text = (options: DatasourceOptions | undefined, key: string): string | undefined => {
  const value = options?.[key];
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
};

const providerName = (options?: DatasourceOptions) =>
  text(options, "provider") ?? providerFromUrl(text(options, "url") ?? text(options, "DATABASE_URL") ?? "");

export class Datasource {
  static defaults = (options: DatasourceOptions): void => {
    specified = { ...options };
  };

  static register(...factories: IDatasourceFactory[]): void {
    for (const factory of factories) registry.set(factory.provider, factory);
  }

  static create(provider?: string, options?: DatasourceOptions): IDatasource {
    const merged = { ...specified, ...options };
    const name = provider ?? providerName(merged);
    if (!name) throw new DatasourceError("Datasource defaults are not specified");
    const factory = registry.get(name);
    if (!factory) throw new ProviderNotInstalledError(name);
    return factory.create(merged);
  }

  static reset(): void {
    registry.clear();
    specified = undefined;
  }

  static registered = (): string[] => [...registry.keys()];
}
