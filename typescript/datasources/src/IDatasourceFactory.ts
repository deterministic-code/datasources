import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { IDatasource } from "./IDatasource.ts";

export interface IDatasourceFactory {
  create(options?: DatasourceOptions): IDatasource;
}
