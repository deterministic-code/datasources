import type { TypeConverter } from "../converters/TypeConverter.ts";
import { TransformMapper } from "./TransformMapper.ts";

export class TransformConverterMapper<T> extends TransformMapper<T, T> {
  constructor(converter: TypeConverter<T, T>, filter?: (value: T) => boolean) {
    super(
      (value) => (filter?.(value) !== false ? converter.convert(value) : value),
      (value) => (filter?.(value) !== false ? converter.reverseConvert(value) : value),
    );
  }
}
