import type { ContactSource } from "../types/generated/datasource/contactSource.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";

export class ContactSourceRepository extends CRUDRepository<ContactSource> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactSource>(connection, { entity: "ContactSource", entityMapper: new DictionaryMapper([["ContactSource", "contact_sources"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
}
