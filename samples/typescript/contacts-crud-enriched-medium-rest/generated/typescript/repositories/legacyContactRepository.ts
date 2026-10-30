import type { LegacyContact } from "../types/generated/views/legacyContact";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<LegacyContact>()
    .add({ entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "legacy_contacts"]]), propertyMappings: new DictionaryMapper([["firstName", "first_name"], ["lastName", "last_name"], ["importedAt", "imported_at"]]), ids: [], createProperties: ["key", "firstName", "lastName", "email", "importedAt"], updateProperties: ["key", "firstName", "lastName", "email", "importedAt"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("sqlite", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], createProperties: ["key", "firstName", "lastName", "email", "importedAt"], updateProperties: ["firstName", "lastName", "email", "importedAt"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("mysql", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], createProperties: ["key", "firstName", "lastName", "email", "importedAt"], updateProperties: ["firstName", "lastName", "email", "importedAt"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("postgres", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), ids: ["key"], createProperties: ["key", "firstName", "lastName", "email", "importedAt"], updateProperties: ["firstName", "lastName", "email", "importedAt"], readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<LegacyContact>()
    .add({ entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "legacy_contacts"]]), propertyMappings: new DictionaryMapper([["firstName", "first_name"], ["lastName", "last_name"], ["importedAt", "imported_at"]]), readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("sqlite", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("mysql", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .add("postgres", { entity: "LegacyContact", entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]), propertyMappings: new DictionaryMapper([["key", "CntID"], ["firstName", "FirstNm"], ["lastName", "LastNm"], ["email", "EmailAddr"], ["importedAt", "ImpDate"]]), readProperties: ["key", "firstName", "lastName", "email", "importedAt"] })
    .for(connection);

export class LegacyContactRepository extends CRUDRepository<LegacyContact> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.createZeros = { key: "", firstName: "", lastName: "" };
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LegacyContact>(connection, readonlyOptions(connection));
}
