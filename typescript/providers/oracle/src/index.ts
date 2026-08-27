import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class OracleDatasource implements DatasourceFactory {
  static readonly factory = new OracleDatasource();
  readonly provider = "oracle";
  create = (_options?: unknown): IDatasource => new StubDatasource("oracle");
}

export default OracleDatasource;
