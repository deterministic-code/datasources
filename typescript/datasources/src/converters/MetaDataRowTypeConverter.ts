import type { Cell, Cells } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import type { Constructable } from "./Constructable.ts";
import type { IMapper } from "../mappers/IMapper.ts";
import { IdentityMapper } from "../mappers/IdentityMapper.ts";

const scalarId = (value: Cell | object | undefined): Cell | undefined => {
  if (value === undefined || value === null || typeof value !== "object")
    return value === undefined ? undefined : (value as Cell);
  const record = value as Record<string, Cell | undefined>;
  return record.Id !== undefined ? record.Id : record.id;
};

export class MetaDataRowTypeConverter<T = Cells> extends TypeConverter<T, Cells> {
  fields: Record<string, TypeConverter<Cell, Cell>>;
  references = new Set<string>();
  referenceColumns: IMapper<string, string> = new IdentityMapper<string>();
  shouldCoerceId = false;

  constructor(fields: Record<string, TypeConverter<Cell, Cell>> = {}) {
    super();
    this.fields = { ...fields };
  }

  convert = (row: Cells, type?: Constructable<T>): T => {
    const blank = type ? new type() : {};
    const cells = Object.fromEntries(
      Object.entries(row).filter(([key, value]) => {
        if (this.references.has(key)) return false;
        const current = (blank as Cells)[key];
        return !(typeof current === "string" && typeof value !== "string" && value !== null);
      }),
    );
    const dest = Object.assign(blank, cells) as T;
    const record = dest as Record<string, Cell | object>;
    for (const [key, value] of Object.entries(row)) {
      if (this.references.has(key)) {
        const current = record[key];
        const target = current !== null && typeof current === "object" && !Array.isArray(current)
          ? current as Record<string, Cell>
          : {};
        const id = "Id" in target ? "Id" : "id";
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
    for (const [key, value] of Object.entries(row as Record<string, Cell | object>)) {
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
        const id = scalarId(value);
        if (id !== undefined && id !== null) cells[this.referenceColumns.map(key)] = this.converted(key, id);
        continue;
      }
      const converter = this.fields[key];
      cells[key] = converter ? converter.reverseConvert(value as Cell) : (value as Cell);
    }
    return cells;
  };

  private converted = (key: string, value: Cell): Cell => {
    const converter = this.fields[key];
    return converter ? converter.reverseConvert(value) : value;
  };
}
