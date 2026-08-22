import { Datasource } from "@datasources/core";
import { JsonDatasource } from "./datasource.ts";

Datasource.use(JsonDatasource);

export {
  JsonCommand,
} from "./command.ts";
export {
  JsonConnection,
  type JsonConnectionOptions,
} from "./connection.ts";
export { JsonDatasource } from "./datasource.ts";
export {
  MemoryFileStore,
  NodeFileStore,
  type FileSystemApi,
  type IFileStore,
} from "./file-store.ts";
export {
  asRecord,
  matches,
  parametersToObject,
  parseFilter,
} from "./filter.ts";
export { JsonRepository } from "./repository.ts";
export { JsonTransaction } from "./transaction.ts";
