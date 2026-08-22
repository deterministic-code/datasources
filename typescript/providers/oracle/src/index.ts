import { createStubDatasource, Datasource } from "@datasources/core";

export const OracleDatasource = createStubDatasource("oracle");
Datasource.use(OracleDatasource);
export default OracleDatasource;
