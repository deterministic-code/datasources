import type { Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";
import { RowConverter } from "./RowConverter.ts";

export class TypeConverter<T extends Cells = Cells> {
  readonly rows = new RowConverter();
  constructor(
    readonly name: string,
    readonly type?: Constructable<T>,
  ) {}
  convert = (row: Cells) => this.rows.convert<T>(row, this.type);
  reverseConvert = (obj: T) => this.rows.reverseConvert(obj);
}
