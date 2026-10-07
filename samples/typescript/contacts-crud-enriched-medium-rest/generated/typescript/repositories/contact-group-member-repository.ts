import type { ContactGroupMember } from "../types/generated/datasource/contactGroupMember.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class ContactGroupMemberRepository extends CRUDRepository<ContactGroupMember> {
  constructor(connection: IConnection) {
    super(connection, { entity: "contact_group_member", entityMapper: new DictionaryMapper([["contact_group_member", "contact_group_members"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contact_id", "contact_group_id"] });
  }
}
