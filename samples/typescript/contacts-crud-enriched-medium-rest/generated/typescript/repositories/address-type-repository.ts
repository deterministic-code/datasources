import type { AddressType } from "../types/generated/datasource/addressType.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";

export class AddressTypeRepository extends CRUDRepository<AddressType> {
  constructor(connection: IConnection) {
    super(connection, { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<AddressType>(connection, { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
}
