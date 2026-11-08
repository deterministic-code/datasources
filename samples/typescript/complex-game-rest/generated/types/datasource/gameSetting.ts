// schema-version: 1.0
/** Type GameSetting. */
export const gameSetting = (init: Partial<GameSetting>): GameSetting => ({ ...init } as GameSetting);
export class GameSetting {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  gameId!: number;
  name!: string;
  value!: string;
}
