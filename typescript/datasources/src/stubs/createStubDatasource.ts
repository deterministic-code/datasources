import { StubDatasource } from "./StubDatasource.ts";

export const createStubDatasource = (provider: string): StubDatasource =>
  new StubDatasource(provider);
