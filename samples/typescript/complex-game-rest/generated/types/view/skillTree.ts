// schema-version: 1.0
/** View SkillTree. */
export const skillTree = (init: Partial<SkillTree>): SkillTree => Object.assign(new SkillTree(), init);
export class SkillTree {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  parentSkillId!: number | null;
  name: string = "";
  children: SkillTree[] = [];
}
