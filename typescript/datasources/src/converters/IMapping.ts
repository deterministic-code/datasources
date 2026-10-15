export interface IMapping<TFrom, TTo> {
  convert(from: TFrom): TTo;
  reverseConvert(to: TTo): TFrom;
}
