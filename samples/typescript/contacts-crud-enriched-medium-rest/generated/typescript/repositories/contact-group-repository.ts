import type { ContactGroup } from "../types/generated/views/contactGroup.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";
import { ContactRepository } from "./contact-repository.ts";
import { ContactGroupMemberRepository } from "./contact-group-member-repository.ts";

export class ContactGroupRepository extends CRUDRepository<ContactGroup> {
  readonly contact: ContactRepository;
  readonly contactGroupMember: ContactGroupMemberRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "contact_groups_base", entityMapper: new DictionaryMapper([["contact_groups_base", "contact_groups"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "name", "description"] });
    this.contact = new ContactRepository(connection);
    this.contactGroupMember = new ContactGroupMemberRepository(connection);
    this.addLinked({ destinationProperty: "members", sourceProperty: "id", relatedKey: "id", association: { parent: "contact_group_id", related: "contact_id" }, type: "list" }, this.contact, this.contactGroupMember);
  }
}
