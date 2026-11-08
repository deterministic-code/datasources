// schema-version: 1.0
import type { Category as CategoriesBase } from "../datasource/category";
/** View Category. */
export const category = (init: Partial<Category>): Category => ({ ...init } as Category);
export interface Category extends CategoriesBase {}
