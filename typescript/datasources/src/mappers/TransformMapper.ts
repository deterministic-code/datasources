import type { IMapper } from "./IMapper.ts";

export class TransformMapper<TFrom, TTo> implements IMapper<TFrom, TTo> {
  constructor(
    readonly map: (value: TFrom) => TTo,
    readonly reverseMap: (value: TTo) => TFrom,
  ) {}
}
