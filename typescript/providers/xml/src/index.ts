import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class XmlDatasource implements DatasourceFactory {
  static readonly factory = new XmlDatasource();
  readonly provider = "xml";
  create = (_options?: unknown): IDatasource => new StubDatasource("xml");
}

export default XmlDatasource;
