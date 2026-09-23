import type { IMapper } from "./IMapper.ts";

export class IdentityMapper<T> implements IMapper<T, T> {
  map = (value: T): T => value;
  reverseMap = (value: T): T => value;
}
