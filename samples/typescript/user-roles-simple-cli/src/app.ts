import {
  ExecuteStoredProcedureStatement,
  FlatToNestedMapper,
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
import { asUserView } from "./as-user-view.ts";
import type { CliCommand } from "./cli-command.ts";
import { columns } from "./columns.ts";
import { passwordHash } from "./password-hash.ts";
import { ident } from "./schema.ts";
import { usersRolesGroupsQuery } from "./users-roles-groups-query.ts";
import { usersView } from "./users-view.ts";

const stamp = (now: () => Date): string => now().toISOString();
const bytes = (value: Uint8Array): Scalar => Buffer.from(value) as unknown as Scalar;

export class App {
  constructor(
    private readonly command: ICommand,
    private readonly now: () => Date = () => new Date(),
    private readonly hash: (password: string) => string = passwordHash,
    private readonly token: () => Uint8Array = () => crypto.getRandomValues(new Uint8Array(16)),
  ) {}

  run = (parsed: CliCommand): Promise<string> =>
    parsed.kind === "add-user"
      ? this.writeUser(parsed.username, parsed.password)
      : parsed.kind === "add-role"
        ? this.writeNamed("roles", "name", parsed.name, "add_role")
        : parsed.kind === "add-group"
          ? this.writeNamed("groups", columns.map("name"), parsed.name, "add_group")
          : parsed.kind === "add-user-role"
            ? this.writeLink(
                parsed.username,
                "roles",
                "name",
                parsed.role,
                "user_roles",
                columns.map("role_id"),
                "add_user_role",
              )
            : parsed.kind === "add-user-group"
              ? this.writeLink(
                  parsed.username,
                  "groups",
                  columns.map("name"),
                  parsed.group,
                  "user_groups",
                  "group_id",
                  "add_user_group",
                )
              : parsed.kind === "enable-user"
                ? this.setEnabled(parsed.username, true)
                : parsed.kind === "disable-user"
                  ? this.setEnabled(parsed.username, false)
                  : parsed.kind === "list-users-roles-groups"
                    ? this.list()
                    : Promise.reject(new Error("empty command"));

  private writeUser = async (username: string, password: string): Promise<string> => {
    const created = stamp(this.now);
    const updated = this.token();
    const hashed = this.hash(password);
    await this.write(
      "users",
      "add_user",
      [
        { name: "username", value: username },
        { name: "password", value: hashed },
        { name: "is_enabled", value: this.flag(true) },
        { name: "created", value: created },
        { name: "updated", value: bytes(updated) },
      ],
      [
        { name: "p_username", value: username },
        { name: "p_password", value: hashed },
        { name: "p_is_enabled", value: true },
        { name: "p_created", value: created },
        { name: "p_updated", value: bytes(updated) },
      ],
    );
    return "";
  };

  private writeNamed = async (
    table: string,
    column: string,
    value: string,
    procedure: string,
  ): Promise<string> => {
    const created = stamp(this.now);
    const updated = this.token();
    await this.write(
      table,
      procedure,
      [
        { name: column, value },
        { name: "is_enabled", value: this.flag(true) },
        { name: "created", value: created },
        { name: "updated", value: bytes(updated) },
      ],
      [
        { name: "p_name", value },
        { name: "p_is_enabled", value: true },
        { name: "p_created", value: created },
        { name: "p_updated", value: bytes(updated) },
      ],
    );
    return "";
  };

  private writeLink = async (
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
    const created = stamp(this.now);
    const updated = this.token();
    const insert = [
      { name: "user_id", value: userId },
      { name: linkColumn, value: rightId },
      { name: "created", value: created },
      { name: "updated", value: bytes(updated) },
    ];
    const call = [
      { name: "p_user_id", value: userId },
      { name: linkColumn === "bad_role_id" ? "p_role_id" : "p_group_id", value: rightId },
      { name: "p_created", value: created },
      { name: "p_updated", value: bytes(updated) },
    ];
    await (this.mysql
      ? execute(this.command, new ExecuteStoredProcedureStatement(procedure, call), call)
      : execute(this.command, new InsertStatement(this.table(linkTable), insert), insert));
    return "";
  };

  private setEnabled = async (username: string, enabled: boolean): Promise<string> => {
    const updated = this.token();
    if (this.mysql) {
      const fields = [
        { name: "p_username", value: username },
        { name: "p_is_enabled", value: this.flag(enabled) },
        { name: "p_updated", value: bytes(updated) },
      ];
      await execute(this.command, new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
      return "";
    }
    const id = await this.id(this.table("users"), "username", username);
    const fields: Parameter[] = [
      { name: "is_enabled", value: this.flag(enabled) },
      { name: "updated", value: bytes(updated) },
    ];
    const where = new WhereClause(new Operator.Equals("id", id));
    await execute(this.command, new UpdateStatement(this.table("users"), fields, where), [...fields, ...where.parameters]);
    return "";
  };

  private list = async (): Promise<string> => {
    const reader = await executeReader(this.command, usersRolesGroupsQuery(this.command.connection.provider));
    const rows = [];
    while (await reader.next()) if (reader.current) rows.push(reader.current);
    return JSON.stringify(asUserView(new FlatToNestedMapper(usersView).map(rows)));
  };

  private id = async (table: string, column: string, value: string): Promise<number> => {
    const where = new WhereClause(new Operator.Equals(column, value));
    const reader = await executeReader(this.command, new QueryStatement(table, ["id"], where), where.parameters);
    if (!(await reader.next()) || reader.current == null)
      throw new Error(`${table} '${value}' not found`);
    const id = reader.current.id;
    return typeof id === "number" ? id : (() => {
      throw new Error(`${table} '${value}' not found`);
    })();
  };

  private write = (
    table: string,
    procedure: string,
    insert: Parameter[],
    call: Parameter[],
  ): Promise<unknown> =>
    this.mysql
      ? execute(this.command, new ExecuteStoredProcedureStatement(procedure, call), call)
      : execute(this.command, new InsertStatement(this.table(table), insert), insert);

  private table = (name: string): string => ident(this.command.connection.provider, name);

  private flag = (value: boolean): Scalar =>
    this.command.connection.provider === "sqlite" ? (value ? 1 : 0) : value;

  private get mysql(): boolean {
    return this.command.connection.provider === "mysql";
  }
}
