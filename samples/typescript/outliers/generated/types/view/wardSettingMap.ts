// schema-version: 1.0
/** View WardSettingMap. */
export const wardSettingMap = (init: Partial<WardSettingMap>): WardSettingMap => Object.assign(new WardSettingMap(), init);
export class WardSettingMap {
  key: string = "";
  value: string = "";
}
