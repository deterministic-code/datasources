// schema-version: 1.0
/** Type GamesBase. */
export const gamesBase = (init: Partial<GamesBase>): GamesBase => Object.assign(new GamesBase(), init);
export class GamesBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
  location: string = "";
  isEnabled: boolean = true;
  isDeleted: boolean = false;
}
