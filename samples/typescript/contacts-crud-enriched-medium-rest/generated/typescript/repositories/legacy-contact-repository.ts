import type { LegacyContact } from "../types/generated/views/legacyContact.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class LegacyContactRepository extends CRUDRepository<LegacyContact> {
  constructor(connection: IConnection) {
    super(connection, { entity: "legacy_contact", entityMapper: new DictionaryMapper([["legacy_contact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["first_name", "FirstNm"], ["last_name", "LastNm"], ["email", "EmailAddr"], ["imported_at", "ImpDate"]]), ids: ["key"], readProperties: ["key", "first_name", "last_name", "email", "imported_at"] });
  }
}
