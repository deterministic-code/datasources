import { createStubDatasource, Datasource } from "@datasources/core";

export const PostgresDatasource = createStubDatasource("postgres");
Datasource.use(PostgresDatasource);
export default PostgresDatasource;
