import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class MongoDbDatasource implements DatasourceFactory {
  static readonly factory = new MongoDbDatasource();
  readonly provider = "mongodb";
  create = (_options?: unknown): IDatasource => new StubDatasource("mongodb");
}

export default MongoDbDatasource;
