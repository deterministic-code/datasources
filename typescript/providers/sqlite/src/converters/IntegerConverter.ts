import { DatasourceError, TypeConverter, type Value } from "@deterministic-code/datasources";

export class IntegerConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => {
    if (typeof from !== "string") return from;
    if (!/^-?\d+$/.test(from)) throw new DatasourceError(`"${from}" is not a long`);
    return Number(from);
  };

  reverseConvert = (to: Value): Value => to;
}
