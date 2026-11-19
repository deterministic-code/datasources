// schema-version: 1.0
/** View GameSettingsMap. */
export const gameSettingsMap = (init: Partial<GameSettingsMap>): GameSettingsMap => Object.assign(new GameSettingsMap(), init);
export class GameSettingsMap {
  key: string = "";
  value: string = "";
}
