import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class PostgresDatasource implements DatasourceFactory {
  static readonly factory = new PostgresDatasource();
  readonly provider = "postgres";
  create = (_options?: unknown): IDatasource => new StubDatasource("postgres");
}

export default PostgresDatasource;
