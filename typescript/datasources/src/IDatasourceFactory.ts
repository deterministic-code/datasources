import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { IDatasource } from "./IDatasource.ts";

export interface IDatasourceFactory {
  readonly provider: string;
  create(options?: DatasourceOptions): IDatasource;
}
