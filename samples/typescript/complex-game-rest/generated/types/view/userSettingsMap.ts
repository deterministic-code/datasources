// schema-version: 1.0
/** View UserSettingsMap. */
export const userSettingsMap = (init: Partial<UserSettingsMap>): UserSettingsMap => Object.assign(new UserSettingsMap(), init);
export class UserSettingsMap {
  key: string = "";
  value: string = "";
}
