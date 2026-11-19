// schema-version: 1.0
/** Type Category. */
export const category = (init: Partial<Category>): Category => Object.assign(new Category(), init);
export class Category {
  id!: number;
  parentCategoryId!: number | null;
  name: string = "";
}
