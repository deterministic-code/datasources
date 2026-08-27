import { Datasource } from "@datasources/core";
import { MemoryDatasource } from "./datasource.ts";

Datasource.use(MemoryDatasource);

export { MemoryCommand } from "./command.ts";
export {
  MemoryConnection,
  type MemoryConnectionOptions,
} from "./connection.ts";
export { MemoryDatasource, MemoryProvider } from "./datasource.ts";
export {
  asRecord,
  matches,
  parametersToObject,
  parseFilter,
} from "./filter.ts";
export { MemoryRepository } from "./repository.ts";
export { MemoryStore, type MemoryRecord } from "./store.ts";
export { MemoryTransaction } from "./transaction.ts";
