export interface LegacyContact {
  key: string;
  first_name: string;
  last_name: string;
  email: string | null;
  imported_at: Date | string | null;
}
