import type { Cells } from "./Cell.ts";

export abstract class TypeConverter<To = Cells, From = Cells> {
  abstract convert(from: From): To;
  abstract reverseConvert(to: To): From;
}
