import type { PhoneType } from "../types/generated/datasource/phoneType";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<PhoneType>()
    .add({ entity: "PhoneType", entityMapper: new DictionaryMapper([["PhoneType", "phone_types"]]), ids: ["id"], createProperties: ["uuid", "version", "name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "PhoneType", entityMapper: new DictionaryMapper([["PhoneType", "phone_types"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "PhoneType", entityMapper: new DictionaryMapper([["PhoneType", "phone_types"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

export class PhoneTypeRepository extends CRUDRepository<PhoneType> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<PhoneType>(connection, options(connection));
}
