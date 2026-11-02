import type { Email } from "../types/generated/views/email";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { EmailAttributeRepository } from "./emailAttributeRepository";
import { LabelRepository } from "./labelRepository";
import { EmailLabelRepository } from "./emailLabelRepository";
import { EmailTypeRepository } from "./emailTypeRepository";
export class EmailRepository extends CRUDRepository<Email> {
  readonly emailAttribute: EmailAttributeRepository;
  readonly label: LabelRepository;
  readonly emailLabel: EmailLabelRepository;
  readonly emailType: EmailTypeRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Email", entityMapper: new DictionaryMapper([["Email", "emails"]]), propertyMappings: new DictionaryMapper([["emailTypeId", "email_type_id"], ["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary"], updateProperties: ["uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary"], readProperties: ["id", "uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, emailTypeId: { type: "integer", nullable: false, hasDefault: false }, contactId: { type: "integer", nullable: false, hasDefault: false }, address: { type: "string", nullable: false, hasDefault: true }, displayName: { type: "string", nullable: false, hasDefault: true }, isPrimary: { type: "boolean", nullable: false, hasDefault: false } }) });
    this.emailAttribute = new EmailAttributeRepository(connection);
    this.label = new LabelRepository(connection);
    this.emailLabel = new EmailLabelRepository(connection);
    this.emailType = new EmailTypeRepository(connection);
    this.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "emailId", relatedProperty: "value", type: "dictionary" }, this.emailAttribute);
    this.addLinked({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", association: { parent: "emailId", related: "labelId" }, type: "list" }, this.label, this.emailLabel);
    this.addEnriched({ destinationProperty: "emailTypeName", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.emailType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Email>(connection, { entity: "Email", entityMapper: new DictionaryMapper([["Email", "emails"]]), propertyMappings: new DictionaryMapper([["emailTypeId", "email_type_id"], ["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"]]), readProperties: ["id", "uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, emailTypeId: { type: "integer", nullable: false, hasDefault: false }, contactId: { type: "integer", nullable: false, hasDefault: false }, address: { type: "string", nullable: false, hasDefault: true }, displayName: { type: "string", nullable: false, hasDefault: true }, isPrimary: { type: "boolean", nullable: false, hasDefault: false } }) });
    const emailAttribute = new EmailAttributeRepository(connection);
    const label = new LabelRepository(connection);
    const emailLabel = new EmailLabelRepository(connection);
    const emailType = new EmailTypeRepository(connection);
    repo.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "emailId", relatedProperty: "value", type: "dictionary" }, emailAttribute);
    repo.addEnriched({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", type: "list" }, label);
    repo.addEnriched({ destinationProperty: "emailTypeName", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, emailType);
    return repo;
  }
}
