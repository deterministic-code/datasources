export interface ContactChangeLog {
  occurredAt: Date | string;
  contactId: number;
  changedBy: string;
  changeSummary: string;
}
