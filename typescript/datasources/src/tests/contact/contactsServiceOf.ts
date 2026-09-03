import type { ICommandMiddleware } from "../../ICommandMiddleware.ts";
import type { IConnection } from "../../IConnection.ts";
import { Repository } from "../../Repository.ts";
import type { Constructable } from "../../converters/Constructable.ts";
import { fieldNamesOf } from "../../createRepositoryBuilderOptions.ts";
import { Address } from "./Address.ts";
import { Contact } from "./Contact.ts";
import { ContactBase } from "./ContactBase.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { ContactsService } from "./ContactsService.ts";
import { Phone } from "./Phone.ts";

export const contactsServiceOf = (
  connection: IConnection,
  middleware?: ICommandMiddleware,
): ContactsService => {
  const of = <T>(type: Constructable<T>, entity: string, columns?: string[]) => {
    const repo = new Repository({
      type,
      entity,
      connection,
      columns,
      assignIdentityOnAdd: false,
    });
    return middleware ? repo.use(middleware) : repo;
  };
  return new ContactsService(
    of(Contact, "contacts_base", fieldNamesOf(ContactBase)),
    of(ContactType, "contact_type"),
    of(ContactSource, "contact_source"),
    of(Address, "address"),
    of(Phone, "phone"),
  );
};
