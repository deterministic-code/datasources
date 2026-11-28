// schema-version: 1.0
/** Type CategoryBase. */
export const categoryBase = (init: Partial<CategoryBase>): CategoryBase => Object.assign(new CategoryBase(), init);
export class CategoryBase {
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
