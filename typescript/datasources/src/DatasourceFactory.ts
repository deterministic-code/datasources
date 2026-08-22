import type { IDatasource } from "./IDatasource.ts";

export interface DatasourceFactory {
  readonly provider: string;
  create(options?: unknown): IDatasource;
}
