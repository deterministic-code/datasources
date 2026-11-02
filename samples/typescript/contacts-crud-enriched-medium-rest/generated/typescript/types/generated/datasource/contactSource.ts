// schema-version: 1.0
/** Type ContactSource. */
export class ContactSource {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name!: string;
  description: string | null;
}
