const uncountable = new Set(["sheep", "fish", "series", "species", "deer", "news"]);
const irregular: Record<string, string> = {
  person: "people",
  child: "children",
  man: "men",
  woman: "women",
  mouse: "mice",
  goose: "geese",
  tooth: "teeth",
  foot: "feet",
  ox: "oxen",
};
const irregularSingular = Object.fromEntries(
  Object.entries(irregular).map(([singular, plural]) => [plural, singular]),
);

const vowel = (c: string) => "aeiou".includes(c);
const consonant = (c: string) => /[a-z]/.test(c) && !vowel(c);
const yies = (word: string) => word.length >= 4 && word.endsWith("ies") && consonant(word[word.length - 4]);
const consonantY = (word: string) => word.length >= 2 && word.endsWith("y") && consonant(word[word.length - 2]);
const sibilant = (word: string) =>
  word.endsWith("ch") ||
  word.endsWith("sh") ||
  word.endsWith("x") ||
  word.endsWith("z") ||
  word.endsWith("ss") ||
  (word.length >= 2 && word.endsWith("s") && !word.endsWith("es") && vowel(word[word.length - 2]));
const sibilantEs = (word: string) => word.length > 2 && word.endsWith("es") && sibilant(word.slice(0, -2));

const restore = (original: string, result: string) =>
  original[0] === original[0].toUpperCase()
    ? result[0].toUpperCase() + result.slice(1)
    : result;

const plural = (word: string) =>
  uncountable.has(word) || word in irregularSingular
    ? word
    : irregular[word] ??
      (yies(word) || sibilantEs(word)
        ? word
        : sibilant(word)
          ? word + "es"
          : consonantY(word)
            ? word.slice(0, -1) + "ies"
            : word.endsWith("s")
              ? word
              : word + "s");

const singular = (word: string) =>
  uncountable.has(word) || word in irregular
    ? word
    : (irregularSingular[word] ??
      (yies(word)
        ? word.slice(0, -3) + "y"
        : sibilantEs(word)
          ? word.slice(0, -2)
          : word.endsWith("s") && !sibilant(word)
            ? word.slice(0, -1)
            : word));

const apply = (word: string, transform: (word: string) => string) =>
  word.length === 0 ? word : restore(word, transform(word.toLowerCase()));

export class PluralizerConverter {
  convert = (from: string) => apply(from, plural);
  reverseConvert = (to: string) => apply(to, singular);
}
