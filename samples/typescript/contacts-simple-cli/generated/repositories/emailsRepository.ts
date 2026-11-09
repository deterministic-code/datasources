import { Email } from "../types/view/email";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { EmailAttributesRepository } from "./emailAttributesRepository";
import { LabelsRepository } from "./labelsRepository";
import { EmailLabelsRepository } from "./emailLabelsRepository";
import { EmailTypesRepository } from "./emailTypesRepository";
export class EmailsRepository extends CRUDRepository<Email> {
  readonly emailAttribute: EmailAttributesRepository;
  readonly label: LabelsRepository;
  readonly emailLabel: EmailLabelsRepository;
  readonly emailType: EmailTypesRepository;
  constructor(connection: IConnection) {
    super(connection, { type: Email, entity: "Email", entityMapper: new DictionaryMapper([["Email", "emails"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"], ["emailTypeId", "email_type_id"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "contactId", "address", "displayName", "isPrimary"], updateProperties: ["uuid", "created", "updated", "version", "contactId", "address", "displayName", "isPrimary"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "address", "displayName", "isPrimary", "emailTypeId"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "integer", nullable: false, hasDefault: false }, address: { type: "string", nullable: false, hasDefault: true }, displayName: { type: "string", nullable: false, hasDefault: true }, isPrimary: { type: "boolean", nullable: false, hasDefault: false }, emailTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: false, hasDefault: true } }) });
    this.emailAttribute = new EmailAttributesRepository(connection);
    this.label = new LabelsRepository(connection);
    this.emailLabel = new EmailLabelsRepository(connection);
    this.emailType = new EmailTypesRepository(connection);
    this.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "emailId", relatedProperty: "value", type: "dictionary" }, this.emailAttribute);
    this.addLinked({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", association: { parent: "emailId", related: "labelId" }, type: "list" }, this.label, this.emailLabel);
    this.addEnriched({ destinationProperty: "emailTypeName", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.emailType);
    this.addEnriched({ destinationProperty: "description", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, this.emailType);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Email>(connection, { type: Email, entity: "Email", entityMapper: new DictionaryMapper([["Email", "emails"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["displayName", "display_name"], ["isPrimary", "is_primary"], ["emailTypeId", "email_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "address", "displayName", "isPrimary", "emailTypeId"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, contactId: { type: "integer", nullable: false, hasDefault: false }, address: { type: "string", nullable: false, hasDefault: true }, displayName: { type: "string", nullable: false, hasDefault: true }, isPrimary: { type: "boolean", nullable: false, hasDefault: false }, emailTypeName: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: false, hasDefault: true } }) });
    const emailAttribute = new EmailAttributesRepository(connection);
    const label = new LabelsRepository(connection);
    const emailLabel = new EmailLabelsRepository(connection);
    const emailType = new EmailTypesRepository(connection);
    repo.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "emailId", relatedProperty: "value", type: "dictionary" }, emailAttribute);
    repo.addEnriched({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", type: "list" }, label);
    repo.addEnriched({ destinationProperty: "emailTypeName", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, emailType);
    repo.addEnriched({ destinationProperty: "description", sourceProperty: "emailTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, emailType);
    return repo;
  }
}
