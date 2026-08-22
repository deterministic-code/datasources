import type { DatasourceFactory } from "./DatasourceFactory.ts";
import type { IDatasource } from "./IDatasource.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

const factories = new Map<string, DatasourceFactory>();

export class Datasource {
  static use(factory: DatasourceFactory): void {
    factories.set(factory.provider, factory);
  }

  static create(provider: string, options?: unknown): IDatasource {
    const factory = factories.get(provider);
    if (!factory) {
      throw new ProviderNotInstalledError(provider);
    }
    return factory.create(options);
  }

  static reset(): void {
    factories.clear();
  }

  static registered(): string[] {
    return [...factories.keys()];
  }
}
