import { createRequire } from "node:module";
import { join } from "node:path";
import type { IDriver } from "./IDriver.ts";
import type { ProviderCatalog } from "./ProviderCatalog.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

type DriverCtor = new () => IDriver;
type DriverModule = Record<string, DriverCtor>;

export class DriverLoader {
  static load(catalog: ProviderCatalog): IDriver {
    try {
      const loaded = createRequire(join(process.cwd(), "package.json"))(catalog.location) as DriverModule;
      const Type = loaded[catalog.module];
      return typeof Type === "function" ? new Type() : (() => { throw new ProviderNotInstalledError(catalog.name); })();
    } catch (error) {
      throw error instanceof ProviderNotInstalledError ? error : new ProviderNotInstalledError(catalog.name);
    }
  }
}
