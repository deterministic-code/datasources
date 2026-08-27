import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class YamlDatasource implements DatasourceFactory {
  static readonly factory = new YamlDatasource();
  readonly provider = "yaml";
  create = (_options?: unknown): IDatasource => new StubDatasource("yaml");
}

export default YamlDatasource;
