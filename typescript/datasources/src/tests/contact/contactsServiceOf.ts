import type { ICommandMiddleware } from "../../ICommandMiddleware.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import type { Constructable } from "../../converters/Constructable.ts";
import type { IMapper } from "../../mappers/IMapper.ts";
import { Address } from "./Address.ts";
import { AddressType } from "./AddressType.ts";
import { columns, contactEntity, snake } from "./contactColumns.ts";
import { Contact } from "./Contact.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { ContactsService } from "./ContactsService.ts";
import { Phone } from "./Phone.ts";
import { PhoneType } from "./PhoneType.ts";

export const contactsServiceOf = (
  connection: IConnection,
  middleware?: ICommandMiddleware,
) => {
  const of = <T>(type: Constructable<T>, entity?: IMapper<string, string>) => {
    const repo = new CRUDRepository(connection, {
      type,
      entityMapper: entity ?? snake,
      propertyMappings: columns,
      ids: ["Id"],
    });
    return middleware ? repo.addMiddleware(middleware) : repo;
  };
  return ContactsService.open(
    of(Contact, contactEntity),
    of(ContactType),
    of(ContactSource),
    of(AddressType),
    of(PhoneType),
    of(Address),
    of(Phone),
  );
};
