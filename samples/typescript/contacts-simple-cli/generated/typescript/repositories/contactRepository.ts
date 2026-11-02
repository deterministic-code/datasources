import type { Contact } from "../types/generated/views/contact";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { AddressRepository } from "./addressRepository";
import { EmailRepository } from "./emailRepository";
import { PhoneRepository } from "./phoneRepository";
import { ContactAttributeRepository } from "./contactAttributeRepository";
import { LabelRepository } from "./labelRepository";
import { ContactLabelRepository } from "./contactLabelRepository";
import { GenderRepository } from "./genderRepository";
export class ContactRepository extends CRUDRepository<Contact> {
  readonly address: AddressRepository;
  readonly email: EmailRepository;
  readonly phone: PhoneRepository;
  readonly contactAttribute: ContactAttributeRepository;
  readonly label: LabelRepository;
  readonly contactLabel: ContactLabelRepository;
  readonly gender: GenderRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["givenName", "given_name"], ["additionalName", "additional_name"], ["familyName", "family_name"], ["nickName", "nick_name"], ["jobTitle", "job_title"], ["genderTypeId", "gender_type_id"], ["isStarred", "is_starred"], ["isFavorite", "is_favorite"], ["isDeleted", "is_deleted"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "genderTypeId", "organization", "isStarred", "isFavorite", "isDeleted"], updateProperties: ["uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "genderTypeId", "organization", "isStarred", "isFavorite", "isDeleted"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "genderTypeId", "organization", "isStarred", "isFavorite", "isDeleted"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, givenName: { type: "string", nullable: false, hasDefault: false }, additionalName: { type: "string", nullable: false, hasDefault: true }, pronunciation: { type: "string", nullable: false, hasDefault: true }, familyName: { type: "string", nullable: false, hasDefault: true }, nickName: { type: "string", nullable: false, hasDefault: true }, jobTitle: { type: "string", nullable: false, hasDefault: true }, genderTypeId: { type: "integer", nullable: false, hasDefault: false }, organization: { type: "string", nullable: false, hasDefault: true }, isStarred: { type: "boolean", nullable: false, hasDefault: false }, isFavorite: { type: "boolean", nullable: false, hasDefault: false }, isDeleted: { type: "boolean", nullable: false, hasDefault: false } }) });
    this.address = new AddressRepository(connection);
    this.email = new EmailRepository(connection);
    this.phone = new PhoneRepository(connection);
    this.contactAttribute = new ContactAttributeRepository(connection);
    this.label = new LabelRepository(connection);
    this.contactLabel = new ContactLabelRepository(connection);
    this.gender = new GenderRepository(connection);
    this.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.address);
    this.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.email);
    this.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.phone);
    this.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, this.contactAttribute);
    this.addLinked({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", association: { parent: "contactId", related: "labelId" }, type: "list" }, this.label, this.contactLabel);
    this.addEnriched({ destinationProperty: "genderTypeName", sourceProperty: "genderTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.gender);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Contact>(connection, { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["givenName", "given_name"], ["additionalName", "additional_name"], ["familyName", "family_name"], ["nickName", "nick_name"], ["jobTitle", "job_title"], ["genderTypeId", "gender_type_id"], ["isStarred", "is_starred"], ["isFavorite", "is_favorite"], ["isDeleted", "is_deleted"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "genderTypeId", "organization", "isStarred", "isFavorite", "isDeleted"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, givenName: { type: "string", nullable: false, hasDefault: false }, additionalName: { type: "string", nullable: false, hasDefault: true }, pronunciation: { type: "string", nullable: false, hasDefault: true }, familyName: { type: "string", nullable: false, hasDefault: true }, nickName: { type: "string", nullable: false, hasDefault: true }, jobTitle: { type: "string", nullable: false, hasDefault: true }, genderTypeId: { type: "integer", nullable: false, hasDefault: false }, organization: { type: "string", nullable: false, hasDefault: true }, isStarred: { type: "boolean", nullable: false, hasDefault: false }, isFavorite: { type: "boolean", nullable: false, hasDefault: false }, isDeleted: { type: "boolean", nullable: false, hasDefault: false } }) });
    const address = new AddressRepository(connection);
    const email = new EmailRepository(connection);
    const phone = new PhoneRepository(connection);
    const contactAttribute = new ContactAttributeRepository(connection);
    const label = new LabelRepository(connection);
    const contactLabel = new ContactLabelRepository(connection);
    const gender = new GenderRepository(connection);
    repo.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, address);
    repo.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, email);
    repo.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phone);
    repo.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, contactAttribute);
    repo.addEnriched({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", type: "list" }, label);
    repo.addEnriched({ destinationProperty: "genderTypeName", sourceProperty: "genderTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, gender);
    return repo;
  }
}
