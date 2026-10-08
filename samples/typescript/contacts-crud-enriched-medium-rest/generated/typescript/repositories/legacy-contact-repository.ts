import type { LegacyContact } from "../types/generated/views/legacyContact.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";

export class LegacyContactRepository extends CRUDRepository<LegacyContact> {
  constructor(connection: IConnection) {
    super(connection, { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LegacyContact>(connection, { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] });
}
