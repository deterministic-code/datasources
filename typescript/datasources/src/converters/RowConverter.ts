import type { Cells } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import { CoreTypes, typeNameOf } from "./CoreTypes.ts";
import type { Constructable } from "./Constructable.ts";
import type { ValueConverter } from "./ValueConverter.ts";

export class RowConverter extends TypeConverter {
  readonly types: Record<string, ValueConverter>;
  readonly fields: Record<string, ValueConverter>;

  constructor(
    types: Record<string, ValueConverter> = {},
    fields: Record<string, ValueConverter> = {},
  ) {
    this.types = { ...CoreTypes, ...types };
    this.fields = { ...fields };
  }

  convert = <T extends Cells>(row: Cells, type?: Constructable<T>): T => {
    const proto = type ? new type() : ({} as T);
    const dest = Object.assign(type ? new type() : {}, row) as T;
    for (const [key, value] of Object.entries(row)) {
      const converter = this.pick(key, typeNameOf(proto[key]));
      if (converter) (dest as Cells)[key] = converter.convert(value);
    }
    return dest;
  };

  reverseConvert = <T extends Cells>(row: T, type?: Constructable<T>): Cells => {
    const proto = type ? new type() : ({} as T);
    return Object.fromEntries(
      Object.entries(row).map(([key, value]) => {
        const converter = this.pick(key, typeNameOf(proto[key]));
        return [key, converter ? converter.reverseConvert(value) : value];
      }),
    );
  };

  private pick = (key: string, type?: string): ValueConverter | undefined =>
    this.fields[key] ?? (type ? this.types[type] : undefined);
}
