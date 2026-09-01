import { describe, expect, it } from "vitest";
import { ChangeCasingConverter, type Casing } from "./ChangeCasingConverter.ts";

describe("ChangeCasingConverter", () => {
  it("changes every casing variation", () => {
    const forms: [Casing, string][] = [
      ["camelCase", "twoWords"],
      ["capitalCase", "Two Words"],
      ["constantCase", "TWO_WORDS"],
      ["dotCase", "two.words"],
      ["kebabCase", "two-words"],
      ["noCase", "two words"],
      ["pascalCase", "TwoWords"],
      ["pascalSnakeCase", "Two_Words"],
      ["pathCase", "two/words"],
      ["sentenceCase", "Two words"],
      ["snakeCase", "two_words"],
      ["trainCase", "Two-Words"],
    ];
    for (const [, from] of forms) {
      for (const [casing, expected] of forms)
        expect(new ChangeCasingConverter(casing).convert(from)).toBe(expected);
      expect(new ChangeCasingConverter("snakeCase").reverseConvert(from)).toBe(from);
    }
    expect(new ChangeCasingConverter("camelCase").convert("XMLParser")).toBe("xmlParser");
    expect(new ChangeCasingConverter("camelCase").convert("html5")).toBe("html5");
    expect(new ChangeCasingConverter("camelCase").convert("v2Name")).toBe("v2Name");
    expect(new ChangeCasingConverter("snakeCase").convert("")).toBe("");
    expect(new ChangeCasingConverter("snakeCase").convert("___")).toBe("");
    expect(new ChangeCasingConverter("snakeCase").convert(" two  words\t")).toBe("two_words");
    expect(new ChangeCasingConverter("camelCase").convert("")).toBe("");
    expect(() => new ChangeCasingConverter("nope" as Casing).convert("a")).toThrow();
  });
});
