import { collectAsync } from "../../collectAsync.ts";
import { ConsoleCommandMiddleware } from "../../ConsoleCommandMiddleware.ts";
import type { Cells } from "../../converters/Cell.ts";
import { FlatToNestedMapper } from "../../mappers/FlatToNestedMapper.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import { Join, SelectStatement, type ISelectStatement, type IWhere } from "../../index.ts";
import { fromContacts, type ContactsView } from "./asContactsView.ts";
import { columns, joinEntity } from "./contactColumns.ts";
import { contactsServiceOf } from "./contactsServiceOf.ts";
import { contactsView } from "./contactsView.ts";

export const contactsJoinSelect = (where?: IWhere): ISelectStatement =>
  new SelectStatement({
    table: "Contact",
    fields: [
      "Contact.Id AS Id",
      "Contact.FirstName AS FirstName",
      "Contact.LastName AS LastName",
      "Contact.Email AS Email",
      "ContactSource.Name AS ContactSourceName",
      "ContactType.Name AS ContactTypeName",
      "Address.Id AS AddressId",
      "Address.Line1 AS Line1",
      "Address.City AS City",
      "AddressType.Name AS AddressTypeName",
      "Phone.Id AS PhoneId",
      "Phone.Number AS Number",
      "PhoneType.Name AS PhoneTypeName",
    ],
    joins: [
      Join.inner("ContactSource", "Contact.ContactSourceId", "ContactSource.Id"),
      Join.inner("ContactType", "Contact.ContactTypeId", "ContactType.Id"),
      Join.left("Address", "Contact.Id", "Address.ContactId"),
      Join.left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
      Join.left("Phone", "Contact.Id", "Phone.ContactId"),
      Join.left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
    ],
    where,
  });

export const contactsParity = async (
  connection: IConnection,
  verbose: boolean,
): Promise<{ eager: ContactsView[]; enriched: ContactsView[] }> => {
  const middleware = new ConsoleCommandMiddleware(undefined, undefined, {
    isVerbose: verbose ? "true" : "false",
  });
  const eager = new FlatToNestedMapper(contactsView).map(
    await collectAsync(
      new CRUDRepository<Cells>(connection, {
        entityMapper: joinEntity,
        propertyMappings: columns,
      })
        .addMiddleware(middleware)
        .read(contactsJoinSelect()),
    ),
  );
  const enriched = fromContacts(
    await collectAsync(contactsServiceOf(connection, verbose ? middleware : undefined).read()),
  );
  return { eager, enriched };
};
