import type { ImportContactsRequest } from "../types/generated/views/importContactsRequest.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ImportContactsRequestRepository extends CRUDRepository<ImportContactsRequest> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ImportContactsRequest", entityMapper: new DictionaryMapper([["ImportContactsRequest", "import_contacts_requests"]]), ids: [], readProperties: ["source", "payload"] });
  }
}
