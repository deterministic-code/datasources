import type { ContactGroupMember } from "../types/generated/datasource/contactGroupMember";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroupMember>()
    .add({ entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), ids: ["id"], createProperties: ["uuid", "version", "contactId", "contactGroupId"], updateProperties: ["uuid", "version", "contactId", "contactGroupId"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .add("mysql", { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), ids: ["id"], createProperties: ["contactId", "contactGroupId"], updateProperties: ["uuid", "version", "contactId", "contactGroupId"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .add("postgres", { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), ids: ["id"], createProperties: ["contactId", "contactGroupId"], updateProperties: ["uuid", "version", "contactId", "contactGroupId"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactGroupMember>()
    .add({ entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .add("mysql", { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .add("postgres", { entity: "ContactGroupMember", entityMapper: new DictionaryMapper([["ContactGroupMember", "contact_group_members"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"], ["contactGroupId", "contact_group_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "contactGroupId"] })
    .for(connection);

export class ContactGroupMemberRepository extends CRUDRepository<ContactGroupMember> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactGroupMember>(connection, readonlyOptions(connection));
}
