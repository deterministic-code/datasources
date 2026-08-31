import { describe, expect, it } from "vitest";
import { ChangeCasingConverter, type Casing } from "./ChangeCasingConverter.ts";
import { PluralizerConverter } from "./PluralizerConverter.ts";
import { RegexConverter } from "./RegexConverter.ts";
import { ReplaceConverter } from "./ReplaceConverter.ts";
import { ToLowerCaseConverter } from "./ToLowerCaseConverter.ts";
import { ToUpperCaseConverter } from "./ToUpperCaseConverter.ts";

describe("case converters", () => {
  it("replaces with regex and literals", () => {
    expect(new RegexConverter(/\d+/g, "").convert("Ada 1 Lovelace")).toBe("Ada  Lovelace");
    expect(new RegexConverter("\\d+", "").convert("Ada 1 Lovelace")).toBe("Ada  Lovelace");
    expect(new RegexConverter(/id_(\d+)/, "id-$1").convert("id_7")).toBe("id-7");
    expect(new RegexConverter(/id_(\d+)/, "id-$1").reverseConvert("id_7")).toBe("id_7");
    expect(new ReplaceConverter("_", " ").convert("Ada_Lovelace")).toBe("Ada Lovelace");
    expect(new ReplaceConverter("_", " ").reverseConvert("Ada Lovelace")).toBe("Ada_Lovelace");
  });

  it("lowers and uppers", () => {
    expect(new ToLowerCaseConverter().convert("ADA")).toBe("ada");
    expect(new ToLowerCaseConverter().reverseConvert("ADA")).toBe("ADA");
    expect(new ToUpperCaseConverter().convert("ada")).toBe("ADA");
    expect(new ToUpperCaseConverter().reverseConvert("ada")).toBe("ada");
  });

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

  it("pluralizes and singularizes", () => {
    const pairs: [string, string][] = [
      ["person", "people"],
      ["child", "children"],
      ["man", "men"],
      ["woman", "women"],
      ["mouse", "mice"],
      ["goose", "geese"],
      ["tooth", "teeth"],
      ["foot", "feet"],
      ["ox", "oxen"],
      ["sheep", "sheep"],
      ["fish", "fish"],
      ["series", "series"],
      ["species", "species"],
      ["deer", "deer"],
      ["news", "news"],
      ["box", "boxes"],
      ["dish", "dishes"],
      ["church", "churches"],
      ["quiz", "quizes"],
      ["kiss", "kisses"],
      ["bus", "buses"],
      ["city", "cities"],
      ["cat", "cats"],
      ["toy", "toys"],
      ["2y", "2ys"],
      ["Person", "People"],
      ["", ""],
    ];
    const converter = new PluralizerConverter();
    for (const [singular, plural] of pairs) {
      expect(converter.convert(singular)).toBe(plural);
      expect(converter.convert(plural)).toBe(plural);
      expect(converter.reverseConvert(plural)).toBe(singular);
      expect(converter.reverseConvert(singular)).toBe(singular);
    }
  });
});
