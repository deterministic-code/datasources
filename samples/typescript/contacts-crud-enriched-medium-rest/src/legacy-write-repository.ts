import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";
import type { LegacyContact } from "../generated/typescript/types/generated/views/legacyContact.ts";

export class LegacyContactWriteRepository extends CRUDRepository<LegacyContact> {
  constructor(connection: IConnection) {
    super(connection, {
      entity: "LegacyContact",
      entityMapper: new DictionaryMapper([["LegacyContact", "OldContactsTbl"]]),
      propertyMappings: new DictionaryMapper([
        ["key", "CntID"],
        ["firstName", "FirstNm"],
        ["lastName", "LastNm"],
        ["email", "EmailAddr"],
        ["importedAt", "ImpDate"],
      ]),
      ids: [],
      readProperties: ["key", "firstName", "lastName", "email", "importedAt"],
    });
  }
}
