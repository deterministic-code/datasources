// schema-version: 1.0
/** Type Category. */
export const category = (init: Partial<Category>): Category => Object.assign(new Category(), init);
export class Category {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  categoryId!: number | null;
  name: string = "";
  description: string = "";
  isEnabled: boolean = true;
  isDeleted: boolean = false;
}
