// schema-version: 1.0
/** View RegionCategoryTree. */
export const regionCategoryTree = (init: Partial<RegionCategoryTree>): RegionCategoryTree => Object.assign(new RegionCategoryTree(), init);
export class RegionCategoryTree {
  regionId!: number;
  categoryId!: number;
  parentRegionId!: number | null;
  parentCategoryId!: number | null;
  name: string = "";
  children: RegionCategoryTree[] = [];
}
