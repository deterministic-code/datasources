import type { Address } from "../types/generated/views/address.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";
import { AddressTypeRepository } from "./address-type-repository.ts";

export class AddressRepository extends CRUDRepository<Address> {
  readonly addressType: AddressTypeRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["addressTypeId", "address_type_id"], ["postalCode", "postal_code"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"] });
    this.addressType = new AddressTypeRepository(connection);
    this.addEnriched({ destinationProperty: "addressTypeName", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.addressType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Address>(connection, { entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["addressTypeId", "address_type_id"], ["postalCode", "postal_code"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"] });
    const addressType = new AddressTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "addressTypeName", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, addressType);
    return repo;
  }
}
