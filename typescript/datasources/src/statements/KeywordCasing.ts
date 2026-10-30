export const KeywordCasing = {
  Upper: "upper",
  Lower: "lower",
} as const;

export type KeywordCasing = (typeof KeywordCasing)[keyof typeof KeywordCasing];
