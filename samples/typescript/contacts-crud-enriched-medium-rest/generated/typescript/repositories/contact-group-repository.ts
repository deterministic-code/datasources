import type { ContactGroup } from "../types/generated/views/contactGroup.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";
import { ContactRepository } from "./contact-repository.ts";
import { ContactGroupMemberRepository } from "./contact-group-member-repository.ts";

export class ContactGroupRepository extends CRUDRepository<ContactGroup> {
  readonly contact: ContactRepository;
  readonly contactGroupMember: ContactGroupMemberRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
    this.contact = new ContactRepository(connection);
    this.contactGroupMember = new ContactGroupMemberRepository(connection);
    this.addLinked({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", association: { parent: "contactGroupId", related: "contactId" }, type: "list" }, this.contact, this.contactGroupMember);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<ContactGroup>(connection, { entity: "ContactGroup", entityMapper: new DictionaryMapper([["ContactGroup", "contact_groups"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
    const contact = new ContactRepository(connection);
    const contactGroupMember = new ContactGroupMemberRepository(connection);
    repo.addEnriched({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", type: "list" }, contact);
    return repo;
  }
}
