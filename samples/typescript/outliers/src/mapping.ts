import {
  ChangeCasingMapper,
  CreateTableStatement,
  CRUDRepository,
  execute,
  PrimaryKey,
  Schema,
  Where,
  eq,
  type IConnection,
  type Response,
} from "@deterministic-code/datasources";
import { mapError, mapLines } from "./format.ts";
import { MappingCell, mappingCell } from "./mappingCell.ts";

const token = "550e8400-e29b-41d4-a716-446655440000";
const opened = new Date("2020-01-02T03:04:05.000Z");
const blob = new Uint8Array([1, 2, 3, 4]);
const casing = new ChangeCasingMapper("camelCase", "snakeCase");
const properties = [
  "id",
  "textToString",
  "textToInteger",
  "textToDecimal",
  "textToDatetime",
  "textToBinary",
  "textToGuid",
  "intToInteger",
  "intToFloat",
  "intToDecimal",
  "intToBoolean",
  "floatToInteger",
  "floatToFloat",
  "floatToDecimal",
  "decimalToInteger",
  "decimalToFloat",
  "decimalToDecimal",
  "boolToBoolean",
  "datetimeToDatetime",
  "binaryToBinary",
  "binaryToGuid",
  "guidToString",
  "guidToGuid",
];

export const runMapping = async (connection: IConnection): Promise<string[]> => {
  const lines: string[] = [];
  try {
    await execute(connection.createCommand(), createTable(connection.provider));
  } catch (error) {
    return mapError("create_table", message(error));
  }
  const cells = new CRUDRepository(connection, options());
  const first = seed(1);
  const created = await capture(lines, "create", () => cells.create(first), first);
  await captureMany(lines, "read", () => cells.read());
  await captureMany(lines, "read_where", () => cells.read(new Where("id", eq, created.id)));
  await capture(lines, "read_single", () => cells.readSingle(), created);
  await capture(
    lines,
    "read_single_where",
    () => cells.readSingle(new Where("id", eq, created.id)),
    created,
  );
  await capture(lines, "read_single_by_id", () => cells.readSingleById(created.id), created);
  await capture(lines, "read_single_by_id_row", () => cells.readSingleById({ id: created.id }), created);
  created.textToString = "grace";
  await captureWrite(lines, "update", () => cells.update(created), () => cells.readSingleById(created.id));
  created.textToString = "lin";
  await captureWrite(
    lines,
    "update_where",
    () => cells.update(created, new Where("id", eq, created.id)),
    () => cells.readSingleById(created.id),
  );
  const extra = await attempt(() => cells.create(mappingCell({ ...seed(2), textToString: "second" })), seed(2));
  await captureWrite(lines, "delete", () => cells.delete(extra), async () => extra);
  await captureWrite(
    lines,
    "delete_where",
    () => cells.delete(created, new Where("id", eq, created.id)),
    async () => created,
  );
  await capture(lines, "create_again", () => cells.create(seed(3)), seed(3));
  return lines;
};

const options = () => ({
  type: MappingCell,
  entity: "MappingCell",
  entityMapper: new ChangeCasingMapper("pascalCase", "snakeCase"),
  propertyMappings: casing,
  ids: ["id"],
  readProperties: properties,
  createProperties: properties,
  updateProperties: properties.filter((name) => name !== "id"),
  schema: new Schema({
    id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    textToString: { type: "string", nullable: false, hasDefault: false },
    textToInteger: { type: "integer", nullable: false, hasDefault: false },
    textToDecimal: { type: "decimal", nullable: false, hasDefault: false },
    textToDatetime: { type: "datetime", nullable: false, hasDefault: false },
    textToBinary: { type: "binary", nullable: false, hasDefault: false },
    textToGuid: { type: "uuid", nullable: false, hasDefault: false },
    intToInteger: { type: "integer", nullable: false, hasDefault: false },
    intToFloat: { type: "float", nullable: false, hasDefault: false },
    intToDecimal: { type: "decimal", nullable: false, hasDefault: false },
    intToBoolean: { type: "boolean", nullable: false, hasDefault: false },
    floatToInteger: { type: "integer", nullable: false, hasDefault: false },
    floatToFloat: { type: "float", nullable: false, hasDefault: false },
    floatToDecimal: { type: "decimal", nullable: false, hasDefault: false },
    decimalToInteger: { type: "integer", nullable: false, hasDefault: false },
    decimalToFloat: { type: "float", nullable: false, hasDefault: false },
    decimalToDecimal: { type: "decimal", nullable: false, hasDefault: false },
    boolToBoolean: { type: "boolean", nullable: false, hasDefault: false },
    datetimeToDatetime: { type: "datetime", nullable: false, hasDefault: false },
    binaryToBinary: { type: "binary", nullable: false, hasDefault: false },
    binaryToGuid: { type: "uuid", nullable: false, hasDefault: false },
    guidToString: { type: "string", nullable: false, hasDefault: false },
    guidToGuid: { type: "uuid", nullable: false, hasDefault: false },
  }),
});

const createTable = (provider: string) => {
  const text = native(provider, "text");
  const integer = native(provider, "integer");
  const floating = native(provider, "float");
  const decimalType = native(provider, "decimal");
  const boolean = native(provider, "boolean");
  const datetime = native(provider, "datetime");
  const binary = native(provider, "binary");
  const guid = native(provider, "guid");
  return new CreateTableStatement(
    "mapping_cell",
    [
      { name: "id", type: integer, identity: true },
      { name: "text_to_string", type: text },
      { name: "text_to_integer", type: text },
      { name: "text_to_decimal", type: text },
      { name: "text_to_datetime", type: text },
      { name: "text_to_binary", type: text },
      { name: "text_to_guid", type: text },
      { name: "int_to_integer", type: integer },
      { name: "int_to_float", type: integer },
      { name: "int_to_decimal", type: integer },
      { name: "int_to_boolean", type: integer },
      { name: "float_to_integer", type: floating },
      { name: "float_to_float", type: floating },
      { name: "float_to_decimal", type: floating },
      { name: "decimal_to_integer", type: decimalType },
      { name: "decimal_to_float", type: decimalType },
      { name: "decimal_to_decimal", type: decimalType },
      { name: "bool_to_boolean", type: boolean },
      { name: "datetime_to_datetime", type: datetime },
      { name: "binary_to_binary", type: binary },
      { name: "binary_to_guid", type: binary },
      { name: "guid_to_string", type: guid },
      { name: "guid_to_guid", type: guid },
    ],
    provider === "sqlite" ? [] : [new PrimaryKey("pk_mapping_cell", ["id"])],
  );
};

const native = (provider: string, source: string): string => {
  if (provider === "mysql") {
    return {
      text: "TEXT",
      integer: "INT",
      float: "DOUBLE",
      decimal: "DECIMAL(28, 14)",
      boolean: "TINYINT(1)",
      datetime: "DATETIME(3)",
      binary: "BLOB",
      guid: "CHAR(36)",
    }[source] ?? "TEXT";
  }
  if (provider === "postgres") {
    return {
      text: "TEXT",
      integer: "INTEGER",
      float: "DOUBLE PRECISION",
      decimal: "NUMERIC",
      boolean: "BOOLEAN",
      datetime: "TIMESTAMPTZ",
      binary: "BYTEA",
      guid: "UUID",
    }[source] ?? "TEXT";
  }
  return {
    text: "TEXT",
    integer: "INTEGER",
    float: "REAL",
    decimal: "NUMERIC",
    boolean: "BOOLEAN",
    datetime: "TEXT",
    binary: "BLOB",
    guid: "TEXT",
  }[source] ?? "TEXT";
};

const seed = (id: number): MappingCell =>
  mappingCell({
    id,
    textToString: "ada",
    textToInteger: 10,
    textToDecimal: "10.50",
    textToDatetime: opened,
    textToBinary: blob,
    textToGuid: token,
    intToInteger: 10,
    intToFloat: 10,
    intToDecimal: "7",
    intToBoolean: true,
    floatToInteger: 10,
    floatToFloat: 10.9,
    floatToDecimal: "10.5",
    decimalToInteger: 10,
    decimalToFloat: 1.2345678901234567,
    decimalToDecimal: "10.50",
    boolToBoolean: true,
    datetimeToDatetime: opened,
    binaryToBinary: blob,
    binaryToGuid: token,
    guidToString: token,
    guidToGuid: token,
  });

const capture = async (
  lines: string[],
  method: string,
  call: () => Promise<MappingCell | undefined>,
  fallback: MappingCell,
): Promise<MappingCell> => {
  try {
    const row = (await call()) ?? fallback;
    lines.push(...mapLines(method, row));
    return row;
  } catch (error) {
    lines.push(...mapError(method, message(error)));
    return fallback;
  }
};

const captureMany = async (
  lines: string[],
  method: string,
  call: () => AsyncIterable<MappingCell>,
): Promise<void> => {
  try {
    for await (const row of call()) lines.push(...mapLines(method, row));
  } catch (error) {
    lines.push(...mapError(method, message(error)));
  }
};

const captureWrite = async (
  lines: string[],
  method: string,
  call: () => Promise<Response>,
  read: () => Promise<MappingCell | undefined>,
): Promise<void> => {
  try {
    await call();
    const row = await read();
    if (!row) lines.push(`map ${method}`);
    else lines.push(...mapLines(method, row));
  } catch (error) {
    lines.push(...mapError(method, message(error)));
  }
};

const attempt = async (
  call: () => Promise<MappingCell>,
  fallback: MappingCell,
): Promise<MappingCell> => {
  try {
    return await call();
  } catch {
    return fallback;
  }
};

const message = (error: unknown): string => (error instanceof Error ? error.message : String(error));
