import type { Value, Cells } from "./Value.ts";
import { TypeConverter } from "./TypeConverter.ts";
import type { Constructable } from "./Constructable.ts";
import type { IMapper } from "../mappers/IMapper.ts";
import { IdentityMapper } from "../mappers/IdentityMapper.ts";
import { identityKey } from "../identity.ts";

const scalarId = (value: Value | object | undefined, names: readonly string[]): Value | undefined => {
  if (value === undefined || value === null || typeof value !== "object")
    return value === undefined ? undefined : (value as Value);
  for (const name of [...names, "Id", "id"]) {
    const key = identityKey(value, name);
    if (key !== value) return key as Value;
  }
  return undefined;
};

export class MetaDataRowTypeConverter<T = Cells> extends TypeConverter<T, Cells> {
  fields: Record<string, TypeConverter<Value, Value>>;
  references = new Set<string>();
  referenceColumns: IMapper<string, string> = new IdentityMapper<string>();
  ids: string[] = [];

  constructor(fields: Record<string, TypeConverter<Value, Value>> = {}) {
    super();
    this.fields = { ...fields };
  }

  convert = (row: Cells, type?: Constructable<T>): T => {
    const blank = type ? new type() : {};
    const cells = Object.fromEntries(
      Object.entries(row).filter(([key]) => !this.references.has(key)),
    );
    const dest = Object.assign(blank, cells) as T;
    const record = dest as Record<string, Value | object>;
    for (const [key, value] of Object.entries(row)) {
      if (this.references.has(key)) {
        const current = record[key];
        const target = current !== null && typeof current === "object" && !Array.isArray(current)
          ? current as Record<string, Value>
          : {};
        const id = this.ids.find((name) => name in target) ?? ("Id" in target ? "Id" : "id");
        target[id] = value;
        record[key] = target;
        continue;
      }
      const converter = this.fields[key];
      if (converter) record[key] = converter.convert(value);
    }
    return dest;
  };

  reverseConvert = (row: T): Cells => {
    const cells: Cells = {};
    for (const [key, value] of Object.entries(row as Record<string, Value | object>)) {
      if (Array.isArray(value)) continue;
      if (
        value !== null &&
        typeof value === "object" &&
        !(value instanceof Date) &&
        !(value instanceof Uint8Array) &&
        !this.references.has(key)
      )
        continue;
      if (this.references.has(key)) {
        const id = scalarId(value, this.ids);
        if (id !== undefined && id !== null) cells[this.referenceColumns.map(key)] = this.converted(key, id);
        continue;
      }
      const converter = this.fields[key];
      cells[key] = converter ? converter.reverseConvert(value as Value) : (value as Value);
    }
    return cells;
  };

  private converted = (key: string, value: Value): Value => {
    const converter = this.fields[key];
    return converter ? converter.reverseConvert(value) : value;
  };
};
