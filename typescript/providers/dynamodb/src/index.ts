import { createStubDatasource, Datasource } from "@datasources/core";

export const DynamoDbDatasource = createStubDatasource("dynamodb");
Datasource.use(DynamoDbDatasource);
export default DynamoDbDatasource;
