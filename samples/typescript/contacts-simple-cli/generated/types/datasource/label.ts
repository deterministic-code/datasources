// schema-version: 1.0
/** Type Label. */
export const label = (init: Partial<Label>): Label => Object.assign(new Label(), init);
export class Label {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
}
