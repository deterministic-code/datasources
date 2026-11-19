// schema-version: 1.0
/** Type Item. */
export const item = (init: Partial<Item>): Item => Object.assign(new Item(), init);
export class Item {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
}
