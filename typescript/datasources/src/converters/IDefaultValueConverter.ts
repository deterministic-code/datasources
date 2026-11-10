export interface IDefaultValueConverter {
  convert(specType: string, text: string): string | null;
  reverseConvert(specType: string, native: string): string;
}
