import { collectAsync } from "../../collectAsync.ts";
import { ConsoleCommandMiddleware } from "../../ConsoleCommandMiddleware.ts";
import type { Cells } from "../../converters/Cell.ts";
import { FlatToNestedMapper } from "../../mappers/FlatToNestedMapper.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import { SelectStatement, type ISelectStatement, type IWhere } from "../../index.ts";
import { fromContacts, type ContactsView } from "./asContactsView.ts";
import { columns, joinEntity } from "./contactColumns.ts";
import { contactsServiceOf } from "./contactsServiceOf.ts";
import { contactReads } from "./contactReads.ts";
import { contactsView } from "./contactsView.ts";

export const contactsJoinSelect = (where?: IWhere): ISelectStatement => {
  const select = contactReads.all().select;
  return where === undefined
    ? select
    : new SelectStatement({
        table: select.table,
        fields: select.fields,
        joins: select.joins,
        where,
      });
};

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
