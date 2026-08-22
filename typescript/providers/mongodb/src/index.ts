import { createStubDatasource, Datasource } from "@datasources/core";

export const MongoDbDatasource = createStubDatasource("mongodb");
Datasource.use(MongoDbDatasource);
export default MongoDbDatasource;
