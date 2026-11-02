import type { AddressType } from "../types/generated/datasource/addressType";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<AddressType>()
    .add({ entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "name", "description"], updateProperties: ["uuid", "created", "updated", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("sqlite", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], createProperties: ["uuid", "version", "name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<AddressType>()
    .add({ entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("sqlite", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "AddressType", entityMapper: new DictionaryMapper([["AddressType", "address_types"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

export class AddressTypeRepository extends CRUDRepository<AddressType> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.createZeros = { name: "" };
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<AddressType>(connection, readonlyOptions(connection));
}
