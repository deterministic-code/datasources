import { DatasourceError, TypeConverter, type Value } from "@deterministic-code/datasources";

const integerText = /^-?\d+(?:\.0+)?$/;

export class IntegerConverter extends TypeConverter<Value, Value> {
  convert = (from: Value): Value => {
    if (typeof from !== "string") return from;
    if (!integerText.test(from)) throw new DatasourceError(`"${from}" is not a long`);
    return Number(from);
  };

  reverseConvert = (to: Value): Value =>
    typeof to === "number" && Number.isInteger(to) ? BigInt(to) : to;
}
