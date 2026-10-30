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

  convertField = (field: SpecField): string =>
    field.type === "string"
      ? field.size == null || field.size === "unlimited"
        ? "TEXT"
        : `VARCHAR(${field.size})`
      : this.convert(field.type);

  reverseConvert = (native: string): string =>
    this.reverse.get(native) ??
    (() => {
      throw new Error(`Unknown native field type: ${native}`);
    })();
}
