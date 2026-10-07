import type { Address } from "../types/generated/views/address.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class AddressRepository extends CRUDRepository<Address> {
  constructor(connection: IConnection) {
    super(connection, { entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["postalCode", "postal_code"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "line1", "line2", "city", "region", "postalCode", "country"] });
  }
}
