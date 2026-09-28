import {
  ColumnDefault,
  CreateTableStatement,
  ForeignKey,
  NotNull,
  PrimaryKey,
  type Column,
  type IDriver,
  type ITableConstraint,
} from "@datasources/core";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const quote = (provider: string, id: string): string =>
  provider === "mysql" ? `\`${id.replaceAll("`", "``")}\`` : `"${id.replaceAll('"', '""')}"`;

export const ident = (provider: string, name: string): string => quote(provider, name);

export const qualified = (provider: string, name: string): string =>
  name.split(".").map((part) => quote(provider, part)).join(".");

export const flag = (provider: string, value: boolean): number | boolean =>
  provider === "sqlite" ? (value ? 1 : 0) : value;

const required = new NotNull();

const stored = (provider: string, driver: IDriver, canonical: string): string =>
  provider === "sqlite" ? driver.columnType(canonical) : canonical;

const idColumn = (provider: string, driver: IDriver): Column =>
  ({
    sqlite: (): Column => ({ name: "id", type: `${driver.columnType("int")} PRIMARY KEY` }),
    mysql: (): Column => ({ name: "id", type: "int", identity: true }),
    postgres: (): Column => ({ name: "id", type: "int", identity: true }),
  }[provider] ??
    (() => {
      throw new Error(`unknown provider ${provider}`);
    }))();

const idKey = (provider: string, name: string): ITableConstraint[] =>
  provider === "sqlite" ? [] : [new PrimaryKey(name, ["id"])];

export const schema = (provider: string, driver: IDriver): CreateTableStatement[] => {
  const q = (name: string) => ident(provider, name);
  const id = idColumn(provider, driver);
  const text = stored(provider, driver, "string");
  const flag = stored(provider, driver, "bool");
  const created = stored(provider, driver, "datetime");
  const updated = stored(provider, driver, "binary");
  const number = stored(provider, driver, "int");
  const enabled = new ColumnDefault("1");
  return [
    new CreateTableStatement(
      q("users"),
      [
        id,
        { name: "username", type: text, notNull: required },
        { name: "password", type: text, notNull: required },
        { name: "is_enabled", type: flag, default: enabled, notNull: required },
        { name: "created", type: created, notNull: required },
        { name: "updated", type: updated, notNull: required },
      ],
      idKey(provider, "pk_users"),
    ),
    new CreateTableStatement(
      q("roles"),
      [
        id,
        { name: "name", type: text, notNull: required },
        { name: "is_enabled", type: flag, default: enabled, notNull: required },
        { name: "created", type: created, notNull: required },
        { name: "updated", type: updated, notNull: required },
      ],
      idKey(provider, "pk_roles"),
    ),
    new CreateTableStatement(
      q("groups"),
      [
        id,
        { name: "ugly_name", type: text, notNull: required },
        { name: "is_enabled", type: flag, default: enabled, notNull: required },
        { name: "created", type: created, notNull: required },
        { name: "updated", type: updated, notNull: required },
      ],
      idKey(provider, "pk_groups"),
    ),
    new CreateTableStatement(
      q("user_groups"),
      [
        id,
        { name: "user_id", type: number, notNull: required },
        { name: "group_id", type: number, notNull: required },
        { name: "created", type: created, notNull: required },
        { name: "updated", type: updated, notNull: required },
      ],
      [
        ...idKey(provider, "pk_user_groups"),
        new ForeignKey("fk_user_groups_user", ["user_id"], q("users"), ["id"]),
        new ForeignKey("fk_user_groups_group", ["group_id"], q("groups"), ["id"]),
      ],
    ),
    new CreateTableStatement(
      q("user_roles"),
      [
        id,
        { name: "user_id", type: number, notNull: required },
        { name: "bad_role_id", type: number, notNull: required },
        { name: "created", type: created, notNull: required },
        { name: "updated", type: updated, notNull: required },
      ],
      [
        ...idKey(provider, "pk_user_roles"),
        new ForeignKey("fk_user_roles_user", ["user_id"], q("users"), ["id"]),
        new ForeignKey("fk_user_roles_role", ["bad_role_id"], q("roles"), ["id"]),
      ],
    ),
  ];
};

export const mysqlProcedures = (): string[] =>
  readFileSync(
    join(dirname(fileURLToPath(import.meta.url)), "../../../user-roles-simple-cli-shared/mysql-procedures.sql"),
    "utf8",
  )
    .split(/^---\s*$/m)
    .map((sql) => sql.trim())
    .filter(Boolean);
