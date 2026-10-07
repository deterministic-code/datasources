import type { Phone } from "../types/generated/views/phone.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class PhoneRepository extends CRUDRepository<Phone> {
  constructor(connection: IConnection) {
    super(connection, { entity: "phones_base", entityMapper: new DictionaryMapper([["phones_base", "phones"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contact_id", "number", "label"] });
  }
}
