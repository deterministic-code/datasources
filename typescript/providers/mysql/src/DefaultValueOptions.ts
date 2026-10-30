export type DefaultValueOptions = {
  newId: () => string;
  now: () => string;
  utcNow: () => string;
  booleanOn: string;
  booleanOff: string;
  hex: (arg: string) => string;
  hexReverse: RegExp;
  quote?: (arg: string) => string;
  quoteReverse?: RegExp;
};
