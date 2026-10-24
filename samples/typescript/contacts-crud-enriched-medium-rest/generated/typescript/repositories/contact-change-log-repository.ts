import type { ContactChangeLog } from "../types/generated/views/contactChangeLog";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class ContactChangeLogRepository extends CRUDRepository<ContactChangeLog> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactChangeLog", entityMapper: new DictionaryMapper([["ContactChangeLog", "contact_change_logs"]]), propertyMappings: new DictionaryMapper([["occurredAt", "occurred_at"], ["contactId", "contact_id"], ["changedBy", "changed_by"], ["changeSummary", "change_summary"]]), ids: ["id"], createProperties: ["occurredAt", "contactId", "changedBy", "changeSummary"], updateProperties: ["occurredAt", "contactId", "changedBy", "changeSummary"], readProperties: ["id", "occurredAt", "contactId", "changedBy", "changeSummary"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactChangeLog>(connection, { entity: "ContactChangeLog", entityMapper: new DictionaryMapper([["ContactChangeLog", "contact_change_logs"]]), propertyMappings: new DictionaryMapper([["occurredAt", "occurred_at"], ["contactId", "contact_id"], ["changedBy", "changed_by"], ["changeSummary", "change_summary"]]), readProperties: ["id", "occurredAt", "contactId", "changedBy", "changeSummary"] });
}
