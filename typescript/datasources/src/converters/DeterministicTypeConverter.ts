import type { IMapping } from "./IMapping.ts";
import {
  type FieldSize,
  type SpecField,
} from "./SpecField.ts";
import type { SpecFieldRules } from "./SpecFieldRules.ts";

const isUnlimited = (size?: FieldSize): boolean =>
  size === "unlimited" || size === undefined || size === null;

const sized = (unlimited: string, render: (n: number) => string, field: SpecField): string =>
  isUnlimited(field.size) ? unlimited : render(Number(field.size));

const charLen = (field: SpecField): number =>
  field.size === undefined || field.size === null ? 1 : Number(field.size);

const decimalPrecisionScale = (field: SpecField): [number, number] | null => {
  const s = field.size;
  if (Array.isArray(s) && s.length === 2) {
    const [p, sc] = s;
    if (!Number.isInteger(p) || !Number.isInteger(sc)) {
      throw new Error(
        `decimal field "${field.name}" size must be [precision, scale] integers; got ${JSON.stringify(s)}`,
      );
    }
    return [p, sc];
  }
  if (s === undefined || s === null) return null;
  throw new Error(
    `decimal field "${field.name}" size must be [precision, scale] tuple; got ${JSON.stringify(s)}`,
  );
};

export class DeterministicTypeConverter implements IMapping<string, string> {
  readonly dictionary: Record<string, string>;
  readonly #reverse: Map<string, string>;
  readonly #rules: SpecFieldRules;

  constructor(dictionary: Record<string, string>, rules: SpecFieldRules) {
    this.dictionary = dictionary;
    this.#rules = rules;
    this.#reverse = new Map();
    for (const [specType, native] of Object.entries(dictionary)) {
      if (!this.#reverse.has(native)) this.#reverse.set(native, specType);
    }
  }

  convert(from: string): string;
  convert(from: SpecField): string;
  convert(from: string | SpecField): string {
    if (typeof from === "string") {
      const native = this.dictionary[from];
      if (native === undefined) throw new Error(`Unknown spec field type: ${from}`);
      return native;
    }
    const field = from;
    const row = this.#rules;
    if (field.type === "string") return sized(row.stringUnlimited, row.stringSized, field);
    if (field.type === "character" && row.character !== undefined) {
      return `${row.character}(${charLen(field)})`;
    }
    if (field.type === "decimal" && row.decimal !== undefined) {
      const rule = row.decimal;
      const ps = decimalPrecisionScale(field);
      if (rule.kind === "required") {
        if (!ps) {
          throw new Error(
            `decimal field "${field.name}" requires size: [precision, scale] for ${row.dialectName ?? "sql"}`,
          );
        }
        return `${rule.type}(${ps[0]}, ${ps[1]})`;
      }
      return ps ? `${rule.type}(${ps[0]}, ${ps[1]})` : rule.fallback;
    }
    if (field.type === "binary") {
      if (row.binarySized !== undefined && row.binaryUnlimited !== undefined) {
        return sized(row.binaryUnlimited, row.binarySized, field);
      }
    }
    return this.convert(field.type);
  }

  reverseConvert = (native: string): string => {
    const spec = this.#reverse.get(native);
    if (spec === undefined) throw new Error(`Unknown native field type: ${native}`);
    return spec;
  };
}
