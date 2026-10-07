import type { Address } from "../types/generated/views/address.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class AddressRepository extends CRUDRepository<Address> {
  constructor(connection: IConnection) {
    super(connection, { entity: "addresses_base", entityMapper: new DictionaryMapper([["addresses_base", "addresses"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contact_id", "line1", "line2", "city", "region", "postal_code", "country"] });
  }
}
