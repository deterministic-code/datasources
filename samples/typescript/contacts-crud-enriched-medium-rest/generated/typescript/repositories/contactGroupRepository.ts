import type { ContactGroup } from "../types/generated/views/contactGroup";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";
import { ContactRepository } from "./contactRepository";
import { ContactGroupMemberRepository } from "./contactGroupMemberRepository";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroup>()
    .add({ entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "name", "description"], updateProperties: ["uuid", "created", "updated", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("sqlite", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), ids: ["id"], createProperties: ["uuid", "version", "name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), ids: ["id"], createProperties: ["name", "description"], updateProperties: ["uuid", "version", "name", "description"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroup>()
    .add({ entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups_bases"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("sqlite", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("mysql", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .add("postgres", { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] })
    .for(connection);

export class ContactGroupRepository extends CRUDRepository<ContactGroup> {
  readonly contact: ContactRepository;
  readonly contactGroupMember: ContactGroupMemberRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.createZeros = { name: "" };
    this.contact = new ContactRepository(connection);
    this.contactGroupMember = new ContactGroupMemberRepository(connection);
    this.addLinked({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", association: { parent: "contactGroupId", related: "contactId" }, type: "list" }, this.contact, this.contactGroupMember);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<ContactGroup>(connection, readonlyOptions(connection));
    const contact = new ContactRepository(connection);
    const contactGroupMember = new ContactGroupMemberRepository(connection);
    repo.addEnriched({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", type: "list" }, contact);
    return repo;
  }
}
