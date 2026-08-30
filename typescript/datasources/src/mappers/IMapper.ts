export interface IMapper<TFrom, TTo> {
  map(value: TFrom): TTo;
  reverseMap(value: TTo): TFrom;
}
