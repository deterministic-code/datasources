export type Casing =
  | "camelCase"
  | "capitalCase"
  | "constantCase"
  | "dotCase"
  | "kebabCase"
  | "noCase"
  | "pascalCase"
  | "pascalSnakeCase"
  | "pathCase"
  | "sentenceCase"
  | "snakeCase"
  | "trainCase";

const cap = (word: string) => word[0].toUpperCase() + word.slice(1);
const sep = (c: string) => c === "_" || c === "." || c === "-" || c === "/" || /\s/.test(c);
const upper = (c: string) => c !== c.toLowerCase();
const lower = (c: string) => c !== c.toUpperCase();
const letter = (c: string) => upper(c) || lower(c);
const digit = (c: string) => c >= "0" && c <= "9";

const words = (text: string): string[] => {
  const parts: string[] = [];
  let start = 0;
  for (let i = 0; i < text.length; i++) {
    const split =
      sep(text[i]) ||
      (i > 0 && lower(text[i - 1]) && upper(text[i])) ||
      (i > 0 && i + 1 < text.length && upper(text[i - 1]) && upper(text[i]) && lower(text[i + 1])) ||
      (i > 0 && letter(text[i - 1]) !== letter(text[i]) && (letter(text[i - 1]) || digit(text[i - 1])) && (letter(text[i]) || digit(text[i])));
    if (!split) continue;
    if (i > start) parts.push(text.slice(start, i));
    start = sep(text[i]) ? i + 1 : i;
  }
  if (start < text.length) parts.push(text.slice(start));
  return parts.map((word) => word.toLowerCase()).filter((word) => word.length > 0);
};

const join = (parts: string[], casing: Casing): string =>
  ({
    camelCase: () => parts.map((word, i) => (i === 0 ? word : cap(word))).join(""),
    capitalCase: () => parts.map(cap).join(" "),
    constantCase: () => parts.map((word) => word.toUpperCase()).join("_"),
    dotCase: () => parts.join("."),
    kebabCase: () => parts.join("-"),
    noCase: () => parts.join(" "),
    pascalCase: () => parts.map(cap).join(""),
    pascalSnakeCase: () => parts.map(cap).join("_"),
    pathCase: () => parts.join("/"),
    sentenceCase: () => parts.map((word, i) => (i === 0 ? cap(word) : word)).join(" "),
    snakeCase: () => parts.join("_"),
    trainCase: () => parts.map(cap).join("-"),
  }[casing]());

export class ChangeCasingConverter {
  constructor(private readonly casing: Casing) {}
  convert = (from: string) => join(words(from), this.casing);
  reverseConvert = (to: string) => to;
}
