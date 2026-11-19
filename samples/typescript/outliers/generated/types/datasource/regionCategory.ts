// schema-version: 1.0
/** Type RegionCategory. */
export const regionCategory = (init: Partial<RegionCategory>): RegionCategory => Object.assign(new RegionCategory(), init);
export class RegionCategory {
  regionId!: number;
  categoryId!: number;
  parentRegionId!: number | null;
  parentCategoryId!: number | null;
  name: string = "";
}
