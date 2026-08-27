import { type DatasourceFactory, type IDatasource, StubDatasource } from "@datasources/core";

export class TextDatasource implements DatasourceFactory {
  static readonly factory = new TextDatasource();
  readonly provider = "text";
  create = (_options?: unknown): IDatasource => new StubDatasource("text");
}

export default TextDatasource;
