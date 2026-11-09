import { Contact } from "../types/view/contact";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { AddressesRepository } from "./addressesRepository";
import { EmailsRepository } from "./emailsRepository";
import { PhonesRepository } from "./phonesRepository";
import { ContactAttributesRepository } from "./contactAttributesRepository";
import { LabelsRepository } from "./labelsRepository";
import { ContactLabelsRepository } from "./contactLabelsRepository";
import { GendersRepository } from "./gendersRepository";
export class ContactsRepository extends CRUDRepository<Contact> {
  readonly address: AddressesRepository;
  readonly email: EmailsRepository;
  readonly phone: PhonesRepository;
  readonly contactAttribute: ContactAttributesRepository;
  readonly label: LabelsRepository;
  readonly contactLabel: ContactLabelsRepository;
  readonly gender: GendersRepository;
  constructor(connection: IConnection) {
    super(connection, { type: Contact, entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["givenName", "given_name"], ["additionalName", "additional_name"], ["familyName", "family_name"], ["nickName", "nick_name"], ["jobTitle", "job_title"], ["isStarred", "is_starred"], ["isFavorite", "is_favorite"], ["isDeleted", "is_deleted"], ["genderTypeId", "gender_type_id"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted"], updateProperties: ["uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted", "genderTypeId"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, givenName: { type: "string", nullable: false, hasDefault: false }, additionalName: { type: "string", nullable: false, hasDefault: true }, pronunciation: { type: "string", nullable: false, hasDefault: true }, familyName: { type: "string", nullable: false, hasDefault: true }, nickName: { type: "string", nullable: false, hasDefault: true }, jobTitle: { type: "string", nullable: false, hasDefault: true }, organization: { type: "string", nullable: false, hasDefault: true }, isStarred: { type: "boolean", nullable: false, hasDefault: false }, isFavorite: { type: "boolean", nullable: false, hasDefault: false }, isDeleted: { type: "boolean", nullable: false, hasDefault: false }, genderTypeName: { type: "string", nullable: false, hasDefault: false } }) });
    this.address = new AddressesRepository(connection);
    this.email = new EmailsRepository(connection);
    this.phone = new PhonesRepository(connection);
    this.contactAttribute = new ContactAttributesRepository(connection);
    this.label = new LabelsRepository(connection);
    this.contactLabel = new ContactLabelsRepository(connection);
    this.gender = new GendersRepository(connection);
    this.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.address);
    this.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.email);
    this.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.phone);
    this.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, this.contactAttribute);
    this.addLinked({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", association: { parent: "contactId", related: "labelId" }, type: "list" }, this.label, this.contactLabel);
    this.addEnriched({ destinationProperty: "genderTypeName", sourceProperty: "genderTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.gender);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Contact>(connection, { type: Contact, entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["givenName", "given_name"], ["additionalName", "additional_name"], ["familyName", "family_name"], ["nickName", "nick_name"], ["jobTitle", "job_title"], ["isStarred", "is_starred"], ["isFavorite", "is_favorite"], ["isDeleted", "is_deleted"], ["genderTypeId", "gender_type_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation", "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted", "genderTypeId"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, givenName: { type: "string", nullable: false, hasDefault: false }, additionalName: { type: "string", nullable: false, hasDefault: true }, pronunciation: { type: "string", nullable: false, hasDefault: true }, familyName: { type: "string", nullable: false, hasDefault: true }, nickName: { type: "string", nullable: false, hasDefault: true }, jobTitle: { type: "string", nullable: false, hasDefault: true }, organization: { type: "string", nullable: false, hasDefault: true }, isStarred: { type: "boolean", nullable: false, hasDefault: false }, isFavorite: { type: "boolean", nullable: false, hasDefault: false }, isDeleted: { type: "boolean", nullable: false, hasDefault: false }, genderTypeName: { type: "string", nullable: false, hasDefault: false } }) });
    const address = new AddressesRepository(connection);
    const email = new EmailsRepository(connection);
    const phone = new PhonesRepository(connection);
    const contactAttribute = new ContactAttributesRepository(connection);
    const label = new LabelsRepository(connection);
    const contactLabel = new ContactLabelsRepository(connection);
    const gender = new GendersRepository(connection);
    repo.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, address);
    repo.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, email);
    repo.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phone);
    repo.addEnriched({ destinationProperty: "attributes", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, contactAttribute);
    repo.addEnriched({ destinationProperty: "labels", sourceProperty: "id", relatedKey: "id", relatedProperty: "name", type: "list" }, label);
    repo.addEnriched({ destinationProperty: "genderTypeName", sourceProperty: "genderTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, gender);
    return repo;
  }
}
