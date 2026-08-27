import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class DynamoDbDatasource implements DatasourceFactory {
  static readonly factory = new DynamoDbDatasource();
  readonly provider = "dynamodb";
  create = (_options?: unknown): IDatasource => new StubDatasource("dynamodb");
}

export default DynamoDbDatasource;
