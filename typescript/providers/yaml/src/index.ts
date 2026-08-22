import { createStubDatasource, Datasource } from "@datasources/core";

export const YamlDatasource = createStubDatasource("yaml");
Datasource.use(YamlDatasource);
export default YamlDatasource;
