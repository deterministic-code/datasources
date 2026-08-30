import type { IMapper } from "./IMapper.ts";

export class StringMapper implements IMapper<string> {
  to = (name: string) => name;
  from = (name: string) => name;
}
