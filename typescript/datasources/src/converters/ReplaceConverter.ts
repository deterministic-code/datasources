export class ReplaceConverter {
  constructor(readonly search: string, readonly replacement: string) {}

  convert = (from: string) => from.replaceAll(this.search, this.replacement);
  reverseConvert = (to: string) => to.replaceAll(this.replacement, this.search);
}
