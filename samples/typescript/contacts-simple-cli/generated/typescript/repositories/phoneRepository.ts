import type { Phone } from "../types/generated/views/phone";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { PhoneAttributeRepository } from "./phoneAttributeRepository";
import { LabelRepository } from "./labelRepository";
import { PhoneLabelRepository } from "./phoneLabelRepository";
import { PhoneTypeRepository } from "./phoneTypeRepository";
export class PhoneRepository extends CRUDRepository<Phone> {
  readonly phoneAttribute: PhoneAttributeRepository;
  readonly label: LabelRepository;
  readonly phoneLabel: PhoneLabelRepository;
  readonly phoneType: PhoneTypeRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["phoneTypeId", "phone_type_id"], ["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"], ["isVerified", "is_verified"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "phoneTypeId", "contactId", "number", "displayName", "extention", "isPrimary", "isVerified"], updateProperties: ["uuid", "created", "updated", "version", "phoneTypeId", "contactId", "number", "displayName", "extention", "isPrimary", "isVerified"], readProperties: ["id", "uuid", "created", "updated", "version", "phoneTypeId", "contactId", "number", "displayName", "extention", "isPrimary", "isVerified"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, phoneTypeId: { type: "integer", nullable: false, hasDefault: false }, contactId: { type: "integer", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, displayName: { type: "string", nullable: false, hasDefault: false }, extention: { type: "string", nullable: false, hasDefault: false }, isPrimary: { type: "boolean", nullable: false, hasDefault: false }, isVerified: { type: "boolean", nullable: false, hasDefault: true } }) });
    this.phoneAttribute = new PhoneAttributeRepository(connection);
    this.label = new LabelRepository(connection);
    this.phoneLabel = new PhoneLabelRepository(connection);
    this.phoneType = new PhoneTypeRepository(connection);
    this.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "phoneId", relatedProperty: "value", type: "dictionary" }, this.phoneAttribute);
    this.addLinked({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", association: { parent: "phoneId", related: "labelId" }, type: "list" }, this.label, this.phoneLabel);
    this.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.phoneType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Phone>(connection, { entity: "Phone", entityMapper: new DictionaryMapper([["Phone", "phones"]]), propertyMappings: new DictionaryMapper([["phoneTypeId", "phone_type_id"], ["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"], ["isVerified", "is_verified"]]), readProperties: ["id", "uuid", "created", "updated", "version", "phoneTypeId", "contactId", "number", "displayName", "extention", "isPrimary", "isVerified"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, phoneTypeId: { type: "integer", nullable: false, hasDefault: false }, contactId: { type: "integer", nullable: false, hasDefault: false }, number: { type: "string", nullable: false, hasDefault: false }, displayName: { type: "string", nullable: false, hasDefault: false }, extention: { type: "string", nullable: false, hasDefault: false }, isPrimary: { type: "boolean", nullable: false, hasDefault: false }, isVerified: { type: "boolean", nullable: false, hasDefault: true } }) });
    const phoneAttribute = new PhoneAttributeRepository(connection);
    const label = new LabelRepository(connection);
    const phoneLabel = new PhoneLabelRepository(connection);
    const phoneType = new PhoneTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "phoneId", relatedProperty: "value", type: "dictionary" }, phoneAttribute);
    repo.addEnriched({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", type: "list" }, label);
    repo.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, phoneType);
    return repo;
  }
}
