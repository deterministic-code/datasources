import type { PhoneType } from "../types/generated/datasource/phoneType.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";

export class PhoneTypeRepository extends CRUDRepository<PhoneType> {
  constructor(connection: IConnection) {
    super(connection, { entity: "PhoneType", entityMapper: new DictionaryMapper([["PhoneType", "phone_types"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<PhoneType>(connection, { entity: "PhoneType", entityMapper: new DictionaryMapper([["PhoneType", "phone_types"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
}
