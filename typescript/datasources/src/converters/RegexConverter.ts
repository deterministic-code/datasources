export class RegexConverter {
  readonly regex: RegExp;
  readonly replacement: string;

  constructor(pattern: string | RegExp, replacement: string) {
    this.regex = typeof pattern === "string"
      ? new RegExp(pattern, "g")
      : new RegExp(pattern.source, pattern.flags.includes("g") ? pattern.flags : `${pattern.flags}g`);
    this.replacement = replacement;
  }

  convert = (from: string) => from.replace(this.regex, this.replacement);
  reverseConvert = (to: string) => to;
}
