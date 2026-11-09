// schema-version: 1.0
/** Type Gender. */
export const gender = (init: Partial<Gender>): Gender => Object.assign(new Gender(), init);
export class Gender {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
}
