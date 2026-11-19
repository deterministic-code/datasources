// schema-version: 1.0
/** Type Skill. */
export const skill = (init: Partial<Skill>): Skill => Object.assign(new Skill(), init);
export class Skill {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  parentSkillId!: number | null;
  name: string = "";
}
