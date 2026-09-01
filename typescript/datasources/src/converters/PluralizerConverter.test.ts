import { describe, expect, it } from "vitest";
import { PluralizerConverter } from "./PluralizerConverter.ts";

describe("PluralizerConverter", () => {
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
