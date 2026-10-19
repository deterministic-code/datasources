import type { ICommand, Scalar } from "@deterministic-code/datasources";
import { addGroup } from "./commands/add-group.ts";
import { addRole } from "./commands/add-role.ts";
import { addUser } from "./commands/add-user.ts";
import { addUserGroup } from "./commands/add-user-group.ts";
import { addUserRole } from "./commands/add-user-role.ts";
import type { CliCommand } from "./commands/cli-command.ts";
import { disableUser } from "./commands/disable-user.ts";
import { enableUser } from "./commands/enable-user.ts";
import { listUsersRolesGroups } from "./commands/list-users-roles-groups.ts";
import { passwordHash } from "./password-hash.ts";

export class App {
  constructor(
    readonly command: ICommand,
    readonly now: () => Date = () => new Date(),
    readonly hash: (password: string) => string = passwordHash,
    readonly token: () => Uint8Array = () => crypto.getRandomValues(new Uint8Array(16)),
    readonly procedures = true,
  ) {}

  run = (parsed: CliCommand): Promise<string> =>
    ({
      "add-user": () => addUser(this, parsed as Extract<CliCommand, { kind: "add-user" }>),
      "add-role": () => addRole(this, parsed as Extract<CliCommand, { kind: "add-role" }>),
      "add-group": () => addGroup(this, parsed as Extract<CliCommand, { kind: "add-group" }>),
      "add-user-role": () => addUserRole(this, parsed as Extract<CliCommand, { kind: "add-user-role" }>),
      "add-user-group": () => addUserGroup(this, parsed as Extract<CliCommand, { kind: "add-user-group" }>),
      "enable-user": () => enableUser(this, parsed as Extract<CliCommand, { kind: "enable-user" }>),
      "disable-user": () => disableUser(this, parsed as Extract<CliCommand, { kind: "disable-user" }>),
      "list-users-roles-groups": () => listUsersRolesGroups(this),
    }[parsed.kind] ?? (() => Promise.reject(new Error("empty command"))))();

  stamp = (): string => this.now().toISOString();

  bytes = (value: Uint8Array): Scalar => Buffer.from(value) as unknown as Scalar;
}
