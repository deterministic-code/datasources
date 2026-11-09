// schema-version: 1.0
/** View ContactChangeLog. */
export const contactChangeLog = (init: Partial<ContactChangeLog>): ContactChangeLog => Object.assign(new ContactChangeLog(), init);
export class ContactChangeLog {
  id!: number;
  occurredAt!: Date;
  contactId!: number;
  changedBy: string = "";
  changeSummary: string = "";
}
