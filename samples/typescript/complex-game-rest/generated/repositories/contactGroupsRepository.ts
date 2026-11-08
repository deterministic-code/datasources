import type { ContactGroup } from "../types/view/contactGroup";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { ContactsRepository } from "./contactsRepository";
import { ContactGroupMembersRepository } from "./contactGroupMembersRepository";
const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroup>()
    .add({ entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), propertyMappings: new DictionaryMapper([]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .add("sqlite", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), propertyMappings: new DictionaryMapper([]), ids: ["id"], createProperties: ["uuid", "created", "version", "name", "description"], updateProperties: ["uuid", "created", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroup>()
    .add({ entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .add("sqlite", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false }, description: { type: "string", nullable: true, hasDefault: false } }) })
    .for(connection);

export class ContactGroupsRepository extends CRUDRepository<ContactGroup> {
  readonly contact: ContactsRepository;
  readonly contactGroupMember: ContactGroupMembersRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.contact = new ContactsRepository(connection);
    this.contactGroupMember = new ContactGroupMembersRepository(connection);
    this.addLinked({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", association: { parent: "contactGroupId", related: "contactId" }, type: "list" }, this.contact, this.contactGroupMember);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<ContactGroup>(connection, readonlyOptions(connection));
    const contact = new ContactsRepository(connection);
    const contactGroupMember = new ContactGroupMembersRepository(connection);
    repo.addEnriched({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", type: "list" }, contact);
    return repo;
  }
}
