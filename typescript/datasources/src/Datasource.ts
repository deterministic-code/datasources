import type { DatasourceFactory } from "./DatasourceFactory.ts";
import type { IDatasource } from "./IDatasource.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

const registry = new Map<string, DatasourceFactory>();

export class Datasource {
  static use(...factories: DatasourceFactory[]): void {
    for (const factory of factories) registry.set(factory.provider, factory);
  }

  static create(provider: string, options?: unknown): IDatasource {
    const factory = registry.get(provider);
    if (!factory) throw new ProviderNotInstalledError(provider);
    return factory.create(options);
  }

  static reset(): void {
    registry.clear();
  }

  static registered = (): string[] => [...registry.keys()];
}
