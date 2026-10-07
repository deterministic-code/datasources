import type { ImportContactsResult } from "../types/generated/views/importContactsResult.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ImportContactsResultRepository extends CRUDRepository<ImportContactsResult> {
  constructor(connection: IConnection) {
    super(connection, { entity: "import_contacts_result", entityMapper: new DictionaryMapper([["import_contacts_result", "import_contacts_results"]]), ids: [], readProperties: ["imported_count", "skipped_count", "failed_count"] });
  }
}
