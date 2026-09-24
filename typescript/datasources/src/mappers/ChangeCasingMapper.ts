import { ChangeCasingConverter, type Casing } from "../converters/ChangeCasingConverter.ts";
import type { IMapper } from "./IMapper.ts";

export class ChangeCasingMapper implements IMapper<string, string> {
  private readonly forward: ChangeCasingConverter;
  private readonly reverse: ChangeCasingConverter;

  constructor(from: Casing, to: Casing) {
    this.forward = new ChangeCasingConverter(to);
    this.reverse = new ChangeCasingConverter(from);
  }

  map = (value: string): string => this.forward.convert(value);
  reverseMap = (value: string): string => this.reverse.convert(value);
}
