import type { Address } from "../types/generated/views/address";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";
import { AddressTypeRepository } from "./address-type-repository";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Address>()
    .add({ entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["addressTypeId", "address_type_id"], ["postalCode", "postal_code"]]), ids: ["id"], createProperties: ["uuid", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], updateProperties: ["uuid", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"] })
    .add("mysql", { entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["addressTypeId", "address_type_id"], ["postalCode", "postal_code"]]), ids: ["id"], createProperties: ["contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], updateProperties: ["uuid", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"] })
    .add("postgres", { entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["addressTypeId", "address_type_id"], ["postalCode", "postal_code"]]), ids: ["id"], createProperties: ["contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], updateProperties: ["uuid", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "line2", "city", "region", "postalCode", "country"] })
    .for(connection);

export class AddressRepository extends CRUDRepository<Address> {
  readonly addressType: AddressTypeRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.addressType = new AddressTypeRepository(connection);
    this.addEnriched({ destinationProperty: "addressTypeName", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.addressType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Address>(connection, options(connection));
    const addressType = new AddressTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "addressTypeName", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, addressType);
    return repo;
  }
}
