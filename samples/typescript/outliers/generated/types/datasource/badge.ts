// schema-version: 1.0
/** Type Badge. */
export const badge = (init: Partial<Badge>): Badge => Object.assign(new Badge(), init);
export class Badge {
  badgeId: string = "";
  code: string = "";
  name: string = "";
}
