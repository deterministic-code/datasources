import { Phone } from "../types/view/phone";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { PhoneTypesRepository } from "./phoneTypesRepository";
const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Phone>()
    .add({ type: Phone, entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phone_bases"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], createProperties: ["contactId", "number", "label"], updateProperties: ["uuid", "contactId", "number", "label"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "number", "label", "phoneTypeId"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "biginteger", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, label: { type: "string", nullable: true, hasDefault: false }, phoneTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .add("sqlite", { type: Phone, entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phone_bases"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), ids: ["id"], createProperties: ["uuid", "created", "version", "contactId", "number", "label"], updateProperties: ["uuid", "created", "version", "contactId", "number", "label"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "number", "label", "phoneTypeId"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "biginteger", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, label: { type: "string", nullable: true, hasDefault: false }, phoneTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Phone>()
    .add({ type: Phone, entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phone_bases"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "number", "label", "phoneTypeId"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "biginteger", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, label: { type: "string", nullable: true, hasDefault: false }, phoneTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .add("sqlite", { type: Phone, entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phone_bases"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["phoneTypeId", "phone_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "number", "label", "phoneTypeId"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "biginteger", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, label: { type: "string", nullable: true, hasDefault: false }, phoneTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .for(connection);

export class PhonesRepository extends CRUDRepository<Phone> {
  readonly phoneType: PhoneTypesRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.phoneType = new PhoneTypesRepository(connection);
    this.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.phoneType);
    this.addEnriched({ destinationProperty: "description", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, this.phoneType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Phone>(connection, readonlyOptions(connection));
    const phoneType = new PhoneTypesRepository(connection);
    repo.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, phoneType);
    repo.addEnriched({ destinationProperty: "description", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, phoneType);
    return repo;
  }
}
