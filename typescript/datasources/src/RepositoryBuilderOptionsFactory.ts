import { DatasourceError } from "./DatasourceError.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";

export class RepositoryBuilderOptionsFactory<T = object> {
  private fallback?: CRUDRepositoryOptions<T>;
  private readonly items = new Map<string, CRUDRepositoryOptions<T>>();

  add(options: CRUDRepositoryOptions<T>): this;
  add(dialect: string, options: CRUDRepositoryOptions<T>): this;
  add(dialectOrOptions: string | CRUDRepositoryOptions<T>, options?: CRUDRepositoryOptions<T>): this {
    typeof dialectOrOptions === "string"
      ? this.items.set(dialectOrOptions, options as CRUDRepositoryOptions<T>)
      : (this.fallback = dialectOrOptions);
    return this;
  }

  for = (provider: string): CRUDRepositoryOptions<T> =>
    this.items.get(provider) ?? this.fallback ?? (() => { throw new DatasourceError("repository options required"); })();
}
