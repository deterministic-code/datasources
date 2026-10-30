import { ChangeCasingConverter, type Casing } from "../converters/ChangeCasingConverter.ts";
import { PluralizerConverter } from "../converters/PluralizerConverter.ts";
import { KeywordCasing } from "./KeywordCasing.ts";

const snake = /^[a-z_][a-z0-9_]*$/;
const pluralizer = new PluralizerConverter();

export class StatementNaming {
  readonly tableNameCasing: Casing;
  readonly fieldNameCasing: Casing;
  readonly constraintNameCasing: Casing;
  readonly triggerNameCasing: Casing;
  readonly keywordCasing: KeywordCasing;
  readonly pluralizeTables: boolean;
  readonly objects: "preserve" | "upper" | "lower";

  static readonly sql = new StatementNaming();

  constructor(options: {
    tableNameCasing?: Casing
    fieldNameCasing?: Casing
    constraintNameCasing?: Casing
    triggerNameCasing?: Casing
    keywordCasing?: KeywordCasing
    pluralizeTables?: boolean
    objects?: "preserve" | "upper" | "lower"
  } = {}) {
    this.tableNameCasing = options.tableNameCasing ?? "snakeCase";
    this.fieldNameCasing = options.fieldNameCasing ?? "snakeCase";
    this.constraintNameCasing = options.constraintNameCasing ?? "snakeCase";
    this.triggerNameCasing = options.triggerNameCasing ?? "snakeCase";
    this.keywordCasing = options.keywordCasing ?? KeywordCasing.Upper;
    this.pluralizeTables = options.pluralizeTables ?? true;
    this.objects = options.objects ?? "preserve";
  }

  isVerbatim = (mapping: string): boolean => !snake.test(mapping);

  physicalStem = (name: string): string =>
    this.isVerbatim(name) ? name : this.pluralizeTables ? lastToken(name) : name;

  convert = (text: string, casing: Casing): string => {
    const words = identifierWords(text);
    if (words.length === 0) return text;
    switch (casing) {
      case "camelCase":
        return words.map((word, i) => (i === 0 ? word : cap(word))).join("");
      case "pascalCase":
        return words.map(cap).join("");
      case "snakeCase":
        return words.join("_");
      case "kebabCase":
        return words.join("-");
      default:
        return new ChangeCasingConverter(casing).convert(text);
    }
  };

  letter = (text: string): string =>
    this.objects === "upper"
      ? text.toUpperCase()
      : this.objects === "lower"
        ? text.toLowerCase()
        : text;

  keyword = (text: string): string =>
    this.keywordCasing === KeywordCasing.Lower ? text.toLowerCase() : text.toUpperCase();
}

const identifierWords = (name: string): string[] =>
  name
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2")
    .replace(/[_\-.]+/g, " ")
    .trim()
    .split(/\s+/)
    .filter((word) => word.length > 0)
    .map((word) => word.toLowerCase());

const cap = (word: string): string => word[0].toUpperCase() + word.slice(1);

const lastToken = (name: string): string => {
  const at = name.lastIndexOf("_");
  return at < 0
    ? pluralizer.convert(name)
    : `${name.slice(0, at + 1)}${pluralizer.convert(name.slice(at + 1))}`;
};
