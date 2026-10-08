// schema-version: 1.0
/** Type ContactSource. */
export interface ContactSource {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  name: string;
  description: string | null;
}
