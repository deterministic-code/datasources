import type { ContactSource } from "../types/generated/datasource/contactSource";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactSource>()
    .add({ entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), ids: ["id"], createProperties: ["uuid", "version", "name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactSource>()
    .add({ entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

export class ContactSourceRepository extends CRUDRepository<ContactSource> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactSource>(connection, readonlyOptions(connection));
}
