// schema-version: 1.0
/** Type ItemLink. */
export const itemLink = (init: Partial<ItemLink>): ItemLink => Object.assign(new ItemLink(), init);
export class ItemLink {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  fromItemId!: number;
  toItemId!: number;
}
