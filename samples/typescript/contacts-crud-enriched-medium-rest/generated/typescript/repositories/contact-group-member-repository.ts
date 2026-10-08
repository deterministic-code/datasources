import type { ContactGroupMember } from "../types/generated/datasource/contactGroupMember.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";

export class ContactGroupMemberRepository extends CRUDRepository<ContactGroupMember> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactGroupMember>(connection, { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] });
}
