import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class MysqlDatasource implements DatasourceFactory {
  static readonly factory = new MysqlDatasource();
  readonly provider = "mysql";
  create = (_options?: unknown): IDatasource => new StubDatasource("mysql");
}

export default MysqlDatasource;
