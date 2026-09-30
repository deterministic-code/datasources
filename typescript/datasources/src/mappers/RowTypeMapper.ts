import type { Cells } from "../converters/Cell.ts";
import { MetaDataRowTypeConverter as RowTypeConverter } from "../converters/MetaDataRowTypeConverter.ts";
import type { TypeConverter } from "../converters/TypeConverter.ts";
import { TransformMapper } from "./TransformMapper.ts";

export class RowTypeMapper<T> extends TransformMapper<Cells, T> {
  constructor(converter: TypeConverter<T, Cells> = new RowTypeConverter()) {
    super(
      (row) => converter.convert(row),
      (item) => converter.reverseConvert(item),
    );
  }
}
