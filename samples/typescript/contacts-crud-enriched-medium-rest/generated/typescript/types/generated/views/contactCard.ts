// schema-version: 1.0
/** View ContactCard. */
export const contactCard = (init: Partial<ContactCard>): ContactCard => ({ ...init } as ContactCard);
export interface ContactCard {
  displayName: string;
}
