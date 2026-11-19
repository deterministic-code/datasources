// schema-version: 1.0
/** View CategoryTree. */
export const categoryTree = (init: Partial<CategoryTree>): CategoryTree => Object.assign(new CategoryTree(), init);
export class CategoryTree {
  id!: number;
  parentCategoryId!: number | null;
  name: string = "";
  children: CategoryTree[] = [];
}
