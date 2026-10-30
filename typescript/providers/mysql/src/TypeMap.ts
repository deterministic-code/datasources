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
      return field.size == null || field.size === "unlimited"
        ? "TEXT"
        : Number(field.size) <= 16383
          ? `VARCHAR(${field.size})`
          : "LONGTEXT";
    if (field.type === "character") return `CHAR(${field.size == null ? 1 : Number(field.size)})`;
    if (field.type === "decimal") {
      if (field.size == null) return "DECIMAL(10, 2)";
      if (Array.isArray(field.size) && field.size.length === 2)
        return `DECIMAL(${field.size[0]}, ${field.size[1]})`;
      throw new Error(`decimal field "${field.name}" size must be [precision, scale] tuple; got ${field.size}`);
    }
    if (field.type === "binary")
      return field.size == null || field.size === "unlimited"
        ? "BLOB"
        : Number(field.size) <= 65535
          ? `VARBINARY(${field.size})`
          : "LONGBLOB";
    return this.convert(field.type);
  };

  reverseConvert = (native: string): string =>
    this.reverse.get(native) ??
    (() => {
      throw new Error(`Unknown native field type: ${native}`);
    })();
}
