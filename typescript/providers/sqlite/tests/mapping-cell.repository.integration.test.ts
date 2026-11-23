import { describe, expect, it, beforeEach } from "vitest";
import {
  CRUDRepository,
  ChangeCasingMapper,
  DatasourceFactory,
  Native,
  Schema,
  Where,
  eq,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "../src/SqliteDriver.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteRowTypeConverters } from "../src/converters/SqliteRowTypeConverters.ts";

class MappingCell {
  id = 0;
  textToString = "";
  textToInteger = 0;
  textToDecimal = "";
  textToDatetime = new Date(0);
  intToInteger = 0;
  intToBoolean = false;
  boolToBoolean = false;
  datetimeToDatetime = new Date(0);
  guidToGuid = "";
}

const opened = new Date("2020-01-02T03:04:05.000Z");
const token = "550e8400-e29b-41d4-a716-446655440000";
const casing = new ChangeCasingMapper("camelCase", "snakeCase");
const properties = [
  "id",
  "textToString",
  "textToInteger",
  "textToDecimal",
  "textToDatetime",
  "intToInteger",
  "intToBoolean",
  "boolToBoolean",
  "datetimeToDatetime",
  "guidToGuid",
];

const open = async () => {
  const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
  await connection.open();
  return connection;
};

const options = (readProperties = properties) => ({
  type: MappingCell,
  entity: "MappingCell",
  entityMapper: casing,
  propertyMappings: casing,
  ids: ["id"],
  readProperties,
  createProperties: readProperties,
  rowTypeConverters: SqliteRowTypeConverters,
  schema: new Schema({
    id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    textToString: { type: "string", nullable: false, hasDefault: false },
    textToInteger: { type: "integer", nullable: false, hasDefault: false },
    textToDecimal: { type: "decimal", nullable: false, hasDefault: false },
    textToDatetime: { type: "datetime", nullable: false, hasDefault: false },
    intToInteger: { type: "integer", nullable: false, hasDefault: false },
    intToBoolean: { type: "boolean", nullable: false, hasDefault: false },
    boolToBoolean: { type: "boolean", nullable: false, hasDefault: false },
    datetimeToDatetime: { type: "datetime", nullable: false, hasDefault: false },
    guidToGuid: { type: "uuid", nullable: false, hasDefault: false },
  }),
});

describe("mapping cell repository", () => {
  beforeEach(() => {
    DatasourceFactory.reset();
    DatasourceFactory.register("sqlite", new SqliteDriver());
  });

  it("creates and reads mapping-cell conversions", async () => {
    const connection = await open();
    await connection.createCommand().executeNative(
      new Native(`
            CREATE TABLE mapping_cell (
              id INTEGER PRIMARY KEY,
              text_to_string TEXT,
              text_to_integer TEXT,
              text_to_decimal TEXT,
              text_to_datetime TEXT,
              int_to_integer INTEGER,
              int_to_boolean INTEGER,
              bool_to_boolean INTEGER,
              datetime_to_datetime TEXT,
              guid_to_guid TEXT
            )
          `),
    );
    const cells = new CRUDRepository(connection, options());
    const created = await cells.create(
      Object.assign(new MappingCell(), {
        id: 1,
        textToString: "ada",
        textToInteger: 10,
        textToDecimal: "10.50",
        textToDatetime: opened,
        intToInteger: 10,
        intToBoolean: true,
        boolToBoolean: true,
        datetimeToDatetime: opened,
        guidToGuid: token,
      }),
    );
    const read = await cells.readSingleById(created.id);
    expect(read?.textToString).toBe("ada");
    expect(read?.textToInteger).toBe(10);
    expect(read?.textToDecimal).toBe("10.5");
    expect(read?.textToDatetime).toEqual(opened);
    expect(read?.intToInteger).toBe(10);
    expect(read?.intToBoolean).toBe(true);
    expect(read?.boolToBoolean).toBe(true);
    expect(read?.datetimeToDatetime).toEqual(opened);
    expect(read?.guidToGuid).toBe(token);
    const found = await cells.readSingle(new Where("id", eq, created.id));
    expect(found?.id).toBe(created.id);
  });

  it("reads zero-fraction dotted numeric text as integer", async () => {
    const connection = await open();
    await connection.createCommand().executeNative(
      new Native("CREATE TABLE mapping_cell (id INTEGER PRIMARY KEY, text_to_integer TEXT)"),
    );
    await connection.createCommand().executeNative(
      new Native("INSERT INTO mapping_cell (id, text_to_integer) VALUES (1, '10.0')"),
    );
    const cells = new CRUDRepository(connection, options(["id", "textToInteger"]));
    expect((await cells.readSingleById(1))?.textToInteger).toBe(10);
  });
});
