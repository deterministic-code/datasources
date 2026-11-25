// schema-version: 1.0
/** Type Ward. */
export const ward = (init: Partial<Ward>): Ward => Object.assign(new Ward(), init);
export class Ward {
  regionId!: number;
  wardId!: number;
  name: string = "";
}
