import { createStubDatasource, Datasource } from "@datasources/core";

export const XmlDatasource = createStubDatasource("xml");
Datasource.use(XmlDatasource);
export default XmlDatasource;
