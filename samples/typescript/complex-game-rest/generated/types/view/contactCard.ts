// schema-version: 1.0
/** View ContactCard. */
export const contactCard = (init: Partial<ContactCard>): ContactCard => Object.assign(new ContactCard(), init);
export class ContactCard {
  displayName: string = "";
}
