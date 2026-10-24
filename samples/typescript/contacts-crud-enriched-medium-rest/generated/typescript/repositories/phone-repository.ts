import type { Phone } from "../types/generated/views/phone";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";
import { PhoneTypeRepository } from "./phone-type-repository";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Phone>()
    .add({ entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], createProperties: ["uuid", "version", "contactId", "phoneTypeId", "number", "label"], updateProperties: ["uuid", "version", "contactId", "phoneTypeId", "number", "label"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .add("mysql", { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], createProperties: ["contactId", "phoneTypeId", "number", "label"], updateProperties: ["uuid", "version", "contactId", "phoneTypeId", "number", "label"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .add("postgres", { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], createProperties: ["contactId", "phoneTypeId", "number", "label"], updateProperties: ["uuid", "version", "contactId", "phoneTypeId", "number", "label"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Phone>()
    .add({ entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .add("mysql", { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .add("postgres", { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"] })
    .for(connection);

export class PhoneRepository extends CRUDRepository<Phone> {
  readonly phoneType: PhoneTypeRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.phoneType = new PhoneTypeRepository(connection);
    this.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.phoneType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Phone>(connection, readonlyOptions(connection));
    const phoneType = new PhoneTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, phoneType);
    return repo;
  }
}
