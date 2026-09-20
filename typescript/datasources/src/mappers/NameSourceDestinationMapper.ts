import type { INameMapper } from "./INameMapper.ts";

export class NameSourceDestinationMapper implements INameMapper {
  constructor(
    readonly source: string,
    readonly destination: string,
  ) {}

  map = (name: string) => name;
  reverseMap = (name: string) => name;
}
