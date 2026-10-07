import type { ContactChangeLog } from "../types/generated/views/contactChangeLog.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ContactChangeLogRepository extends CRUDRepository<ContactChangeLog> {
  constructor(connection: IConnection) {
    super(connection, { entity: "contact_change_log", entityMapper: new DictionaryMapper([["contact_change_log", "contact_change_logs"]]), ids: ["id"], readProperties: ["id", "occurred_at", "contact_id", "changed_by", "change_summary"] });
  }
}
