import type { Phone } from "../types/generated/views/phone.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";
import { PhoneTypeRepository } from "./phone-type-repository.ts";

export class PhoneRepository extends CRUDRepository<Phone> {
  readonly phoneType: PhoneTypeRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] });
    this.phoneType = new PhoneTypeRepository(connection);
    this.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.phoneType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Phone>(connection, { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] });
    const phoneType = new PhoneTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, phoneType);
    return repo;
  }
}
