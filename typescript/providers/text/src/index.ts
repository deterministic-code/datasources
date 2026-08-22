import { createStubDatasource, Datasource } from "@datasources/core";

export const TextDatasource = createStubDatasource("text");
Datasource.use(TextDatasource);
export default TextDatasource;
