// schema-version: 1.0
/** View BadgeSettingMap. */
export const badgeSettingMap = (init: Partial<BadgeSettingMap>): BadgeSettingMap => Object.assign(new BadgeSettingMap(), init);
export class BadgeSettingMap {
  key: string = "";
  value: string = "";
}
