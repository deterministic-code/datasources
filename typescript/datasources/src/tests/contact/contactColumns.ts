import { ChangeCasingMapper } from "../../mappers/ChangeCasingMapper.ts";
import { DictionaryMapper } from "../../mappers/DictionaryMapper.ts";
import type { IMapper } from "../../mappers/IMapper.ts";
import { TransformMapper } from "../../mappers/TransformMapper.ts";

export const snake = new ChangeCasingMapper("pascalCase", "snakeCase");

const references = new DictionaryMapper<string>([
  ["Contact", "contact_id"],
  ["AddressType", "address_type_id"],
  ["ContactType", "contact_type_id"],
  ["ContactSource", "contact_source_id"],
  ["PhoneType", "phone_type_id"],
  ["Line1", "line1"],
  ["Line2", "line2"],
]);

export const columns: IMapper<string, string> = new TransformMapper(
  (name) => {
    const mapped = references.map(name);
    return mapped !== name ? mapped : snake.map(name);
  },
  (name) => {
    const from = references.reverseMap(name);
    return from !== name ? from : snake.reverseMap(name);
  },
);

export const contactEntity = new DictionaryMapper<string>([["Contact", "contacts_base"]]);
