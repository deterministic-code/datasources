import type { ImportContactsResult } from "../types/generated/views/importContactsResult.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ImportContactsResultRepository extends CRUDRepository<ImportContactsResult> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ImportContactsResult", entityMapper: new DictionaryMapper([["ImportContactsResult", "import_contacts_results"]]), propertyMappings: new DictionaryMapper([["importedCount", "imported_count"], ["skippedCount", "skipped_count"], ["failedCount", "failed_count"]]), ids: [], readProperties: ["importedCount", "skippedCount", "failedCount"] });
  }
}
