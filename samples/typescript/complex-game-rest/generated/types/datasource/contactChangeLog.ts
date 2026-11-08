// schema-version: 1.0
/** Type ContactChangeLog. */
export const contactChangeLog = (init: Partial<ContactChangeLog>): ContactChangeLog => ({ ...init } as ContactChangeLog);
export interface ContactChangeLog {
  id: number;
  occurredAt: Date;
  contactId: number;
  changedBy: string;
  changeSummary: string;
}
