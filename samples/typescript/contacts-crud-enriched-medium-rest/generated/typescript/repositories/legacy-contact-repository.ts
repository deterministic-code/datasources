import type { LegacyContact } from "../types/generated/views/legacyContact";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class LegacyContactRepository extends CRUDRepository<LegacyContact> {
  constructor(connection: IConnection) {
    super(connection, { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], createProperties: ["key", "firstName", "lastName", "email", "importedAt"], updateProperties: ["firstName", "lastName", "email", "importedAt"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LegacyContact>(connection, { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), readProperties: ["key", "firstName", "lastName", "email", "importedAt"] });
}
