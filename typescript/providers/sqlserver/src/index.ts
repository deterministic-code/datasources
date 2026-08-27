import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class SqlServerDatasource implements DatasourceFactory {
  static readonly factory = new SqlServerDatasource();
  readonly provider = "sqlserver";
  create = (_options?: unknown): IDatasource => new StubDatasource("sqlserver");
}

export default SqlServerDatasource;
