// schema-version: 1.0
import type { ContactGroupsBase } from "../datasource/contactGroupsBase";
import type { Contact } from "./contact";
/** View ContactGroup. */
export const contactGroup = (init: Partial<ContactGroup>): ContactGroup => ({ ...init } as ContactGroup);
export interface ContactGroup extends ContactGroupsBase {
  members: Contact[];
}
