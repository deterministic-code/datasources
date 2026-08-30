import type { ISourceDestinationMapper } from "./ISourceDestinationMapper.ts";
import { StringMapper } from "./StringMapper.ts";

export class StringSourceDestinationMapper
  extends StringMapper
  implements ISourceDestinationMapper
{
  constructor(
    readonly source: string,
    readonly destination: string,
  ) {
    super();
  }
}
