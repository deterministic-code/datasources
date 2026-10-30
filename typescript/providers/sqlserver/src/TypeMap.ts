import type { IMapping, SpecField } from "@deterministic-code/datasources";

export class TypeMap implements IMapping<string, string> {
  readonly dictionary: Record<string, string>;
  private readonly reverse = new Map<string, string>();

  constructor(dictionary: Record<string, string>) {
    this.dictionary = dictionary;
    for (const [spec, native] of Object.entries(dictionary)) {
      if (!this.reverse.has(native)) this.reverse.set(native, spec);
    }
  }

  convert = (from: string): string =>
    this.dictionary[from] ??
    (() => {
      throw new Error(`Unknown spec field type: ${from}`);
    })();

  convertField = (field: SpecField): string => {
    if (field.type === "string")
      return field.size == null || field.size === "unlimited" ? "NVARCHAR(MAX)" : `NVARCHAR(${field.size})`;
    if (field.type === "character") return `NCHAR(${field.size == null ? 1 : Number(field.size)})`;
    if (field.type === "decimal") {
      if (field.size == null) return "DECIMAL";
      if (Array.isArray(field.size) && field.size.length === 2)
        return `DECIMAL(${field.size[0]}, ${field.size[1]})`;
      throw new Error(`decimal field "${field.name}" size must be [precision, scale] tuple; got ${field.size}`);
    }
    if (field.type === "binary")
      return field.size == null || field.size === "unlimited" ? "VARBINARY(MAX)" : `VARBINARY(${field.size})`;
    return this.convert(field.type);
  };

  reverseConvert = (native: string): string =>
    this.reverse.get(native) ??
    (() => {
      throw new Error(`Unknown native field type: ${native}`);
    })();
}
