// schema-version: 1.0
/** Type Group. */
export const group = (init: Partial<Group>): Group => ({ ...init } as Group);
export class Group {
  id!: number;
  uuid: string = crypto.randomUUID();
  created!: Date;
  updated: string = Buffer.from([0]).toString("base64");
  name!: string;
  description!: string;
  isEnabled: boolean = true;
}
