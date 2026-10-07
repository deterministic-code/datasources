import type { ContactChangeLog } from "../types/generated/views/contactChangeLog.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ContactChangeLogRepository extends CRUDRepository<ContactChangeLog> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactChangeLog", entityMapper: new DictionaryMapper([["ContactChangeLog", "contact_change_logs"]]), propertyMappings: new DictionaryMapper([["occurredAt", "occurred_at"], ["contactId", "contact_id"], ["changedBy", "changed_by"], ["changeSummary", "change_summary"]]), ids: ["id"], readProperties: ["id", "occurredAt", "contactId", "changedBy", "changeSummary"] });
  }
}
