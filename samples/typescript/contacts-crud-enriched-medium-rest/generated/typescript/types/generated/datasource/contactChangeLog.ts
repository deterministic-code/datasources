// schema-version: 1.0
/** Type ContactChangeLog. */
export interface ContactChangeLog {
  id: number;
  occurredAt: Date;
  contactId: number;
  changedBy: string;
  changeSummary: string;
}
