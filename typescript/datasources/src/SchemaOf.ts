import type { Constructable } from "./converters/Constructable.ts";
import { DeterministicTypeMapper } from "./mappers/DeterministicTypeMapper.ts";
import { Schema, type SchemaColumn } from "./Schema.ts";

export class SchemaOf {
  static from = <T>(type: Constructable<T>): Schema => {
    if (type === Object) return new Schema({});
    const mapper = new DeterministicTypeMapper();
    const instance = new type() as Record<string, unknown>;
    const columns: Record<string, SchemaColumn> = {};
    for (const [name, value] of Object.entries(instance)) {
      const mapped = mapper.map(kindOf(value));
      if (mapped)
        columns[name] = { type: mapped, nullable: false, hasDefault: false };
    }
    return new Schema(columns);
  };
}

const kindOf = (value: unknown): string =>
  value instanceof Date
    ? "Date"
    : value instanceof Uint8Array
      ? "Uint8Array"
      : typeof value === "boolean"
        ? "boolean"
        : typeof value === "number"
          ? "number"
          : typeof value === "bigint"
            ? "bigint"
            : "";
