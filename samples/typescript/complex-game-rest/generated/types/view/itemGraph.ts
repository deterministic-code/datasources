// schema-version: 1.0
/** View ItemGraph. */
export const itemGraph = (init: Partial<ItemGraph>): ItemGraph => Object.assign(new ItemGraph(), init);
export class ItemGraph {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  edges: ItemGraph[] = [];
}
