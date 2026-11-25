import { beforeAll, describe, expect, it } from "vitest";
import {
  CRUDRepository,
  createCRUDRepositoryOptions,
  DictionaryMapper,
  InsertStatement,
  Native,
  ParameterDirection,
  ReturnStatement,
  ExecuteStoredProcedureStatement,
  StatementTextRenderer,
} from "@deterministic-code/datasources";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/sql/generate-sql";
import { MysqlCommand } from "../src/command.ts";
import { MysqlConnection } from "../src/connection.ts";
import { MysqlStatementRenderer } from "../src/MysqlStatementRenderer.ts";
import type { Mysql2Module } from "../src/i-mysql-engine.ts";
import "../src/register-sql-generate.ts";

const dialect = loadDialect(
  "mysql",
  (statement) => new StatementTextRenderer(t => new MysqlStatementRenderer(t)).render(statement),
);
let url = (process.env.MYSQL_URL ??=
  "mysql://ds:ds@127.0.0.1:3306/datasources");

beforeAll(() => {
  ensureCompose("mysql");
  url = process.env.MYSQL_URL ?? url;
}, 120_000);

async function fillOutputParameter(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
    BEGIN
      SET result = start_n + 1;
    END
  `));
  const start = { name: "start_n", value: 4 };
  const result = { name: "result", direction: ParameterDirection.Output };
  const call = await command.executeStatement(
    new ExecuteStoredProcedureStatement("bump", [start, result]),
    [start, result],
  );
  expect(call.parameters?.[1]?.value).toBe(5);
}

async function callProcedureReturnsTwoResults(command: MysqlCommand) {
  await command.executeNative(new Native(`
    CREATE PROCEDURE two_ids()
    BEGIN
      SELECT 1 AS id;
      SELECT 2 AS id;
    END
  `));
  await using reader = await command.executeReaderStatement(new ExecuteStoredProcedureStatement("two_ids", []));
  const first: { id: number }[] = [];
  while (await reader.next()) first.push(reader.current as { id: number });
  const moved = await reader.nextResult();
  const second: { id: number }[] = [];
  while (await reader.next()) second.push(reader.current as { id: number });
  const again = await reader.nextResult();
  expect(moved).toBe(true);
  expect(again).toBe(false);
  expect(first[0]?.id).toBe(1);
  expect(second[0]?.id).toBe(2);
}

const replaceDatabase = (base: string, name: string): string => {
  const parsed = new URL(base);
  parsed.pathname = `/${name}`;
  return parsed.toString();
};

const isolate = async (base: string) => {
  const mysql = (await import("mysql2/promise")) as Mysql2Module;
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = await mysql.createConnection({
    uri: base,
    namedPlaceholders: true,
  });
  await admin.query(`CREATE DATABASE \`${name}\``);
  await admin.end();
  const isolated = replaceDatabase(base, name);
  return {
    url: isolated,
    cleanup: async () => {
      const drop = await mysql.createConnection({
        uri: base,
        namedPlaceholders: true,
      });
      await drop.query(`DROP DATABASE \`${name}\``);
      await drop.end();
    },
  };
};

describe("shared sql specs", () => {
  for (const file of specFiles()) {
    it(file.split("/").pop()!, async () => {
      const isolated = await isolate(url);
      try {
        await runSpec(file, dialect, {
          openSession: async () => {
            const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
            return { connection, command: new MysqlCommand(connection) };
          },
          closedCommand: () =>
            new MysqlCommand(new MysqlConnection()),
        });
      } finally {
        await isolated.cleanup();
      }
    });
  }
});

describe("execute reader", () => {
  it("enumerates two result sets", async () => {
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await using reader = await command.executeReaderNative(new Native("SELECT 1 AS id; SELECT 2 AS id"));
      const first: { id: number }[] = [];
      while (await reader.next()) first.push(reader.current as { id: number });
      const moved = await reader.nextResult();
      const second: { id: number }[] = [];
      while (await reader.next()) second.push(reader.current as { id: number });
      const again = await reader.nextResult();
      expect(moved).toBe(true);
      expect(again).toBe(false);
      expect(first[0]?.id).toBe(1);
      expect(second[0]?.id).toBe(2);
    } finally {
      await isolated.cleanup();
    }
  });
});

describe("insert returning", () => {
  it("create puts the inserted id on the item", async () => {
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await command.executeNative(
        new Native("CREATE TABLE items (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT)"),
      );
      const native = await command.executeNative(new Native("INSERT INTO items (name) VALUES ('ada')"));
      expect(native.result).toBeNull();
      const fields = [{ name: "name", value: "bea" }];
      const inserted = await command.executeStatement(new InsertStatement("items", fields, "id"), fields);
      expect(inserted.result).not.toBeNull();
      expect(inserted.result).toBe(2);
      class Item {
        id!: number;
        name = "";
      }
      const repo = new CRUDRepository(connection, {
        ...createCRUDRepositoryOptions(Item),
        ids: ["id"],
        entity: "items",
      });
      const created = await repo.create(Object.assign(new Item(), { name: "cee" }));
      expect(created.id).toBeGreaterThan(0);
      const next = await repo.create(Object.assign(new Item(), { name: "dee" }));
      expect(next.id).toBeGreaterThan(created.id);
    } finally {
      await isolated.cleanup();
    }
  });

  it("create then child insert uses the parent id", async () => {
    class Contact {
      id!: number;
      name = "";
    }
    class Address {
      id!: number;
      contactId!: number;
      line1 = "";
    }
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await command.executeNative(
        new Native(
          "CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT NOT NULL); CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTO_INCREMENT, contact_id INTEGER NOT NULL, line1 TEXT NOT NULL)",
        ),
      );
      const contacts = new CRUDRepository(connection, {
        ...createCRUDRepositoryOptions(Contact),
        ids: ["id"],
        entity: "contacts",
        createProperties: ["name"],
      });
      const addresses = new CRUDRepository(connection, {
        ...createCRUDRepositoryOptions(Address),
        ids: ["id"],
        entity: "addresses",
        createProperties: ["contactId", "line1"],
        propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]),
      });
      const created = await contacts.create(Object.assign(new Contact(), { name: "ada" }));
      await addresses.create(Object.assign(new Address(), { contactId: created.id, line1: "12 Engine" }));
      await using reader = await command.executeReaderNative(new Native("SELECT contact_id FROM addresses"));
      expect(await reader.next()).toBe(true);
      expect(reader.current?.contact_id).toBe(created.id);
    } finally {
      await isolated.cleanup();
    }
  });
});

const runBatches = async (command: MysqlCommand, sql: string) => {
  for (const batch of sql.split(/\nGO\n/)) {
    const text = batch.trim();
    if (text !== "") await command.executeNative(new Native(text));
  }
};

describe("generateSql identifier length", () => {
  it("applies fitted names from generated migrations", async () => {
    const entity = "game_journey_setting_category_preference_option_value";
    const files = await generateSql(
      {
        reader: memoryReader({
          "types.yaml": `types:
  - category_type:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - ${entity}:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
        - updated:
            type: datetime
        - category_type_id:
            type: integer
            references: category_type.id
        - external_reference_code:
            type: string
`,
          "datasource.yaml": `includes:
  - types:
      filter: tag == "datasource_type"
types:
  - ${entity}:
      indexes:
        - category_type_id:
            fields:
              - category_type_id
`,
        }),
        settings: {},
      },
      ["mysql"],
    );
    const sqlOf = (suffix: string) => {
      const entry = files.find((item) => item.filename.endsWith(suffix));
      expect(entry).toBeDefined();
      return "contents" in entry! ? String(entry.contents) : entry!.content;
    };
    const up = sqlOf("mysql/migrations/0001_initial_up.sql");
    const down = sqlOf("mysql/migrations/0001_initial_down.sql");
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await runBatches(command, up);
      await using reader = await command.executeReaderNative(
        new Native(`
          SELECT CHAR_LENGTH(INDEX_NAME) AS n FROM information_schema.STATISTICS WHERE TABLE_SCHEMA = DATABASE()
          UNION ALL
          SELECT CHAR_LENGTH(CONSTRAINT_NAME) FROM information_schema.TABLE_CONSTRAINTS WHERE TABLE_SCHEMA = DATABASE()
          UNION ALL
          SELECT CHAR_LENGTH(TRIGGER_NAME) FROM information_schema.TRIGGERS WHERE TRIGGER_SCHEMA = DATABASE()
        `),
      );
      while (await reader.next()) {
        const n = Number((reader.current as { n: number }).n);
        expect(n).toBeLessThanOrEqual(64);
      }
      await runBatches(command, down);
    } finally {
      await isolated.cleanup();
    }
  });
});

describe("return and output parameters", () => {
  it("reads OUT and function results", async () => {
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await fillOutputParameter(command);
      await callProcedureReturnsTwoResults(command);
      await command.executeNative(new Native(`
        CREATE FUNCTION add_one(n INT) RETURNS INT DETERMINISTIC
        RETURN n + 1
      `));
      const n = { name: "n", value: 4 };
      const ret = await command.executeStatement(
        new ReturnStatement("add_one", [n]),
        [n],
      );
      expect(ret.result).toBe(5);
    } finally {
      await isolated.cleanup();
    }
  });
});
