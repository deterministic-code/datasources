import type { Phone } from "../types/generated/views/phone.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class PhoneRepository extends CRUDRepository<Phone> {
  constructor(connection: IConnection) {
    super(connection, { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "number", "label"] });
  }
}
