import {
  ConsoleCommandMiddleware,
  Datasource,
  ExecuteStoredProcedureStatement,
  FlatToNestedMapper,
  InsertStatement,
  Join,
  Operator,
  OrderBy,
  SelectStatement,
  TransformMapper,
  UpdateStatement,
  Where,
  execute,
  executeReader,
  type ICommand,
  type IMapper,
  type Parameter,
  type TypeMap,
} from "@deterministic-code/datasources";
import bcrypt from "bcryptjs";
import { existsSync, readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

export const usage = `add-user <username> <password>
add-role <name>
add-group <name>
add-user-role <username> <role>
add-user-group <username> <group>
enable-user <username>
disable-user <username>
list-users-roles-groups`;

export const names = new TransformMapper(
  (name: string) => ({ name: "ugly_name", role_id: "bad_role_id" }[name] ?? name),
  (name: string) => ({ ugly_name: "name", bad_role_id: "role_id" }[name] ?? name),
);

export type UserView = {
  id: number;
  username: string;
  is_enabled: boolean;
  created: string;
  roles: { id: number; name: string; is_enabled: boolean }[];
  groups: { id: number; name: string; is_enabled: boolean }[];
};

export const usersView: TypeMap<UserView> = {
  key: "id",
  fields: {
    id: "id",
    username: "username",
    is_enabled: "is_enabled",
    created: "created",
    roles: {
      key: "role_id",
      fields: { id: "role_id", name: "role_name", is_enabled: "role_is_enabled" },
    },
    groups: {
      key: "group_id",
      fields: { id: "group_id", name: "group_name", is_enabled: "group_is_enabled" },
    },
  },
};

export const listQuery = () =>
  new SelectStatement({
    table: "users",
    fields: [
      "users.id AS id",
      "users.username AS username",
      "users.is_enabled AS is_enabled",
      "users.created AS created",
      "roles.id AS role_id",
      "roles.name AS role_name",
      "roles.is_enabled AS role_is_enabled",
      "groups.id AS group_id",
      "groups.ugly_name AS group_name",
      "groups.is_enabled AS group_is_enabled",
    ],
    joins: [
      Join.left("user_roles", "users.id", "user_roles.user_id"),
      Join.left("roles", "user_roles.bad_role_id", "roles.id"),
      Join.left("user_groups", "users.id", "user_groups.user_id"),
      Join.left("groups", "user_groups.group_id", "groups.id"),
    ],
    orderBy: [OrderBy.asc("users.id"), OrderBy.asc("roles.id"), OrderBy.asc("groups.id")],
  });

export const passwordHash = (password: string): string => bcrypt.hashSync(password, 4);

export const parse = (args: string[]) =>
  ({
    "add-user": () =>
      args.length === 3 ? { kind: "add-user" as const, values: [args[1]!, args[2]!] } : fail("Usage: add-user <username> <password>"),
    "add-role": () => (args.length === 2 ? { kind: "add-role" as const, values: [args[1]!] } : fail("Usage: add-role <name>")),
    "add-group": () => (args.length === 2 ? { kind: "add-group" as const, values: [args[1]!] } : fail("Usage: add-group <name>")),
    "add-user-role": () =>
      args.length === 3 ? { kind: "add-user-role" as const, values: [args[1]!, args[2]!] } : fail("Usage: add-user-role <username> <role>"),
    "add-user-group": () =>
      args.length === 3 ? { kind: "add-user-group" as const, values: [args[1]!, args[2]!] } : fail("Usage: add-user-group <username> <group>"),
    "enable-user": () =>
      args.length === 2 ? { kind: "enable-user" as const, values: [args[1]!] } : fail("Usage: enable-user <username>"),
    "disable-user": () =>
      args.length === 2 ? { kind: "disable-user" as const, values: [args[1]!] } : fail("Usage: disable-user <username>"),
    "list-users-roles-groups": () =>
      args.length === 1 ? { kind: "list-users-roles-groups" as const, values: [] as string[] } : fail(`Unknown command: ${args[0]}\n${usage}`),
  }[args[0] ?? ""] ?? fail(args[0] ? `Unknown command: ${args[0]}\n${usage}` : usage))();

const fail = (message: string): never => {
  throw new Error(message);
};

const created = (): Parameter => ({ name: "created", value: new Date() });
const updated = (name: string): Parameter => ({ name, value: new Uint8Array([0]) });

const write = async (
  command: ICommand,
  table: string,
  procedure: string,
  insert: Parameter[],
  call: Parameter[],
  procedures: boolean,
) => {
  if (procedures && command.connection.provider === "mysql")
    await execute(command, new ExecuteStoredProcedureStatement(procedure, call), call);
  else await execute(command, new InsertStatement(table, insert), insert);
  return "";
};

const lookupId = async (command: ICommand, table: string, column: string, value: string): Promise<number> => {
  const where = new Where(new Operator.Equals(column, value));
  const reader = await executeReader(command, new SelectStatement({ table, fields: ["id"], where }), where.parameters);
  try {
    if (!(await reader.next()) || reader.current == null) throw new Error(`${table} '${value}' not found`);
    const id = reader.current.id;
    return typeof id === "number" ? id : fail(`${table} '${value}' not found`);
  } finally {
    await reader.dispose();
  }
};

const link = async (
  command: ICommand,
  username: string,
  rightTable: string,
  rightColumn: string,
  rightValue: string,
  linkTable: string,
  linkColumn: string,
  procedure: string,
  procedures: boolean,
) => {
  const userId = await lookupId(command, "users", "username", username);
  const rightId = await lookupId(command, rightTable, rightColumn, rightValue);
  return write(
    command,
    linkTable,
    procedure,
    [
      { name: "user_id", value: userId },
      { name: linkColumn, value: rightId },
      created(),
    ],
    [
      { name: "p_user_id", value: userId },
      { name: linkColumn === "bad_role_id" ? "p_role_id" : "p_group_id", value: rightId },
      { name: "p_created", value: created().value },
      updated("p_updated"),
    ],
    procedures,
  );
};

const setEnabled = async (command: ICommand, username: string, enabled: boolean, procedures: boolean) => {
  if (procedures && command.connection.provider === "mysql") {
    const fields: Parameter[] = [
      { name: "p_username", value: username },
      { name: "p_is_enabled", value: enabled },
      updated("p_updated"),
    ];
    await execute(command, new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
    return "";
  }
  const id = await lookupId(command, "users", "username", username);
  const fields: Parameter[] = [{ name: "is_enabled", value: enabled }];
  const where = new Where(new Operator.Equals("id", id));
  await execute(command, new UpdateStatement("users", fields, where), [...fields, ...where.parameters]);
  return "";
};

const list = async (command: ICommand) => {
  const reader = await executeReader(command, listQuery());
  const rows = [];
  try {
    while (await reader.next()) if (reader.current) rows.push(reader.current);
  } finally {
    await reader.dispose();
  }
  return JSON.stringify(new UserViewMapper().map(new FlatToNestedMapper(usersView).map(rows)));
};

class UserViewMapper implements IMapper<Iterable<UserView>, UserView[]> {
  map(rows: Iterable<UserView>): UserView[] {
    return [...rows]
      .map((row) => ({
        id: this.id(row.id),
        username: this.text(row.username),
        is_enabled: this.flag(row.is_enabled),
        created: this.text(row.created),
        roles: [...(row.roles ?? [])]
          .map((role) => ({ id: this.id(role.id), name: this.text(role.name), is_enabled: this.flag(role.is_enabled) }))
          .sort((left, right) => left.id - right.id),
        groups: [...(row.groups ?? [])]
          .map((group) => ({ id: this.id(group.id), name: this.text(group.name), is_enabled: this.flag(group.is_enabled) }))
          .sort((left, right) => left.id - right.id),
      }))
      .sort((left, right) => left.id - right.id);
  }

  reverseMap(_value: UserView[]): Iterable<UserView> {
    throw new Error("ReverseMap is not supported");
  }

  private id(value: number | string): number {
    if (typeof value === "number") return value;
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }

  private text(value: string | Date | null | undefined): string {
    if (value instanceof Date) return value.toISOString();
    return value == null ? "" : `${value}`;
  }

  private flag(value: boolean | number): boolean {
    if (typeof value === "boolean") return value;
    return value !== 0;
  }
}

export const run = async (
  command: ICommand,
  args: string[],
  hash: (password: string) => string = passwordHash,
  procedures = true,
): Promise<string> => {
  const parsed = parse(args);
  const [first, second] = parsed.values;
  switch (parsed.kind) {
    case "add-user":
      return write(
        command,
        "users",
        "add_user",
        [
          { name: "username", value: first! },
          { name: "password", value: hash(second!) },
          { name: "is_enabled", value: true },
          created(),
        ],
        [
          { name: "p_username", value: first! },
          { name: "p_password", value: hash(second!) },
          { name: "p_is_enabled", value: true },
          { name: "p_created", value: created().value },
          updated("p_updated"),
        ],
        procedures,
      );
    case "add-role":
      return write(
        command,
        "roles",
        "add_role",
        [
          { name: "name", value: first! },
          { name: "is_enabled", value: true },
          created(),
        ],
        [
          { name: "p_name", value: first! },
          { name: "p_is_enabled", value: true },
          { name: "p_created", value: created().value },
          updated("p_updated"),
        ],
        procedures,
      );
    case "add-group":
      return write(
        command,
        "groups",
        "add_group",
        [
          { name: names.map("name"), value: first! },
          { name: "is_enabled", value: true },
          created(),
        ],
        [
          { name: "p_name", value: first! },
          { name: "p_is_enabled", value: true },
          { name: "p_created", value: created().value },
          updated("p_updated"),
        ],
        procedures,
      );
    case "add-user-role":
      return link(command, first!, "roles", "name", second!, "user_roles", names.map("role_id"), "add_user_role", procedures);
    case "add-user-group":
      return link(command, first!, "groups", names.map("name"), second!, "user_groups", "group_id", "add_user_group", procedures);
    case "enable-user":
      return setEnabled(command, first!, true, procedures);
    case "disable-user":
      return setEnabled(command, first!, false, procedures);
    case "list-users-roles-groups":
      return list(command);
    default:
      throw new Error("empty command");
  }
};

const env = (key: string): string => {
  const value = process.env[key]?.trim();
  if (value) return value;
  if (!existsSync(".env")) return "";
  for (const line of readFileSync(".env", "utf8").split(/\r?\n/)) {
    const i = line.indexOf("=");
    if (i > 0 && line.slice(0, i).trim() === key) return line.slice(i + 1).trim();
  }
  return "";
};

const main = async (args: string[]): Promise<number> => {
  try {
    await using ds = Datasource.default;
    const connection = await ds.createConnection();
    await connection.open();
    const command = await ds.createCommand(connection);
    const frozen = env("CLI_NOW");
    const now = frozen ? () => new Date(frozen) : () => new Date();
    command.addMiddleware(new ConsoleCommandMiddleware((line) => console.error(line), now));
    process.stdout.write(await run(command, args));
    return 0;
  } catch (error) {
    process.stderr.write(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href)
  process.exit(await main(process.argv.slice(2)));
