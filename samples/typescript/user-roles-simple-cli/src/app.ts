import {
  ExecuteStoredProcedureStatement,
  InsertStatement,
  Operator,
  QueryStatement,
  UpdateStatement,
  WhereClause,
  execute,
  executeReader,
  type ICommand,
  type Parameter,
  type Scalar,
} from "@datasources/core";
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
import { ident } from "./schema.ts";

export class App {
  constructor(
    readonly command: ICommand,
    readonly now: () => Date = () => new Date(),
    readonly hash: (password: string) => string = passwordHash,
    readonly token: () => Uint8Array = () => crypto.getRandomValues(new Uint8Array(16)),
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

  writeNamed = async (table: string, column: string, value: string, procedure: string): Promise<string> => {
    const created = this.stamp();
    const updated = this.token();
    await this.write(
      table,
      procedure,
      [
        { name: column, value },
        { name: "is_enabled", value: this.flag(true) },
        { name: "created", value: created },
        { name: "updated", value: this.bytes(updated) },
      ],
      [
        { name: "p_name", value },
        { name: "p_is_enabled", value: true },
        { name: "p_created", value: created },
        { name: "p_updated", value: this.bytes(updated) },
      ],
    );
    return "";
  };

  writeLink = async (
    username: string,
    rightTable: string,
    rightColumn: string,
    rightValue: string,
    linkTable: string,
    linkColumn: string,
    procedure: string,
  ): Promise<string> => {
    const userId = await this.id(this.table("users"), "username", username);
    const rightId = await this.id(this.table(rightTable), rightColumn, rightValue);
    const created = this.stamp();
    const updated = this.token();
    const insert = [
      { name: "user_id", value: userId },
      { name: linkColumn, value: rightId },
      { name: "created", value: created },
      { name: "updated", value: this.bytes(updated) },
    ];
    const call = [
      { name: "p_user_id", value: userId },
      { name: linkColumn === "bad_role_id" ? "p_role_id" : "p_group_id", value: rightId },
      { name: "p_created", value: created },
      { name: "p_updated", value: this.bytes(updated) },
    ];
    await (this.mysql
      ? execute(this.command, new ExecuteStoredProcedureStatement(procedure, call), call)
      : execute(this.command, new InsertStatement(this.table(linkTable), insert), insert));
    return "";
  };

  setEnabled = async (username: string, enabled: boolean): Promise<string> => {
    const updated = this.token();
    if (this.mysql) {
      const fields = [
        { name: "p_username", value: username },
        { name: "p_is_enabled", value: this.flag(enabled) },
        { name: "p_updated", value: this.bytes(updated) },
      ];
      await execute(this.command, new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
      return "";
    }
    const id = await this.id(this.table("users"), "username", username);
    const fields: Parameter[] = [
      { name: "is_enabled", value: this.flag(enabled) },
      { name: "updated", value: this.bytes(updated) },
    ];
    const where = new WhereClause(new Operator.Equals("id", id));
    await execute(this.command, new UpdateStatement(this.table("users"), fields, where), [...fields, ...where.parameters]);
    return "";
  };

  id = async (table: string, column: string, value: string): Promise<number> => {
    const where = new WhereClause(new Operator.Equals(column, value));
    const reader = await executeReader(this.command, new QueryStatement(table, ["id"], where), where.parameters);
    if (!(await reader.next()) || reader.current == null)
      throw new Error(`${table} '${value}' not found`);
    const id = reader.current.id;
    return typeof id === "number" ? id : (() => {
      throw new Error(`${table} '${value}' not found`);
    })();
  };

  write = (table: string, procedure: string, insert: Parameter[], call: Parameter[]): Promise<unknown> =>
    this.mysql
      ? execute(this.command, new ExecuteStoredProcedureStatement(procedure, call), call)
      : execute(this.command, new InsertStatement(this.table(table), insert), insert);

  table = (name: string): string => ident(this.command.connection.provider, name);

  flag = (value: boolean): Scalar =>
    this.command.connection.provider === "sqlite" ? (value ? 1 : 0) : value;

  get mysql(): boolean {
    return this.command.connection.provider === "mysql";
  }
}
