import { createStubDatasource, Datasource } from "@datasources/core";

export const SqlServerDatasource = createStubDatasource("sqlserver");
Datasource.use(SqlServerDatasource);
export default SqlServerDatasource;
