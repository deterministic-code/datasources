import { PluralizerConverter } from "../converters/PluralizerConverter.ts";
import type { IMapper } from "./IMapper.ts";

export class PluralizerMapper implements IMapper<string, string> {
  private readonly pluralizer = new PluralizerConverter();

  map = (value: string): string => this.pluralizer.convert(value);
  reverseMap = (value: string): string => this.pluralizer.reverseConvert(value);
}
