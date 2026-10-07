import type { ImportContactsRequest } from "../types/generated/views/importContactsRequest.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ImportContactsRequestRepository extends CRUDRepository<ImportContactsRequest> {
  constructor(connection: IConnection) {
    super(connection, { entity: "import_contacts_request", entityMapper: new DictionaryMapper([["import_contacts_request", "import_contacts_requests"]]), ids: [], readProperties: ["source", "payload"] });
  }
}
