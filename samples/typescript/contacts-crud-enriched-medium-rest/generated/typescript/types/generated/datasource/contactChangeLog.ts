export interface ContactChangeLog {
  occurred_at: Date | string;
  contact_id: number;
  changed_by: string;
  change_summary: string;
}
