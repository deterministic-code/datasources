import type { ISourceDestinationMapper } from "./ISourceDestinationMapper.ts";
import { NameMapper } from "./NameMapper.ts";

export class NameSourceDestinationMapper
  extends NameMapper
  implements ISourceDestinationMapper
{
  constructor(
    readonly source: string,
    readonly destination: string,
  ) {
    super();
  }
}
