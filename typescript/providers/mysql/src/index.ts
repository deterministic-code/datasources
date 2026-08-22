import { createStubDatasource, Datasource } from "@datasources/core";

export const MysqlDatasource = createStubDatasource("mysql");
Datasource.use(MysqlDatasource);
export default MysqlDatasource;
