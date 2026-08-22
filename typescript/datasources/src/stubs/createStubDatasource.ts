import { StubDatasource } from "./StubDatasource.ts";

export function createStubDatasource(provider: string): StubDatasource {
  return new StubDatasource(provider);
}
