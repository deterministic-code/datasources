// schema-version: 1.0
/** Type Role. */
export const role = (init: Partial<Role>): Role => Object.assign(new Role(), init);
export class Role {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
  isEnabled: boolean = true;
}
