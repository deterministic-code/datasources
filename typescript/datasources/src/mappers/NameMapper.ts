import type { IMapper } from "./IMapper.ts";

export class NameMapper implements IMapper<string, string> {
  map = (name: string) => name;
  reverseMap = (name: string) => name;
}
