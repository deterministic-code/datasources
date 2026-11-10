import type { IDefaultValueConverter } from "./IDefaultValueConverter.ts";

export class EmptyDefaultValueConverter implements IDefaultValueConverter {
  static readonly instance = new EmptyDefaultValueConverter();
  convert = (_specType: string, _text: string): string | null => null;
  reverseConvert = (specType: string, native: string): string => {
    throw new Error(`Native default '${native}' is not a valid ${specType} default`);
  };
}

export const emptyDefaultValueConverter = EmptyDefaultValueConverter.instance;
