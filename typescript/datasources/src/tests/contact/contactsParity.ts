import { collectAsync } from "../../collectAsync.ts";
import { ConsoleCommandMiddleware } from "../../ConsoleCommandMiddleware.ts";
import type { Cells } from "../../converters/Cell.ts";
import { FlatToNestedMapper } from "../../mappers/FlatToNestedMapper.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import type { ContactsView } from "./asContactsView.ts";
import { contactsJoinQuery } from "./contactsJoinQuery.ts";
import { contactsServiceOf } from "./contactsServiceOf.ts";
import { contactsView } from "./contactsView.ts";

export const contactsParity = async (
  connection: IConnection,
  verbose: boolean,
): Promise<{ eager: ContactsView[]; enriched: ContactsView[] }> => {
  const middleware = new ConsoleCommandMiddleware(undefined, undefined, {
    isVerbose: verbose ? "true" : "false",
  });
  const eager = new FlatToNestedMapper(contactsView).map(
    await collectAsync(
      new CRUDRepository<Cells>({ connection, entity: "contacts_base" })
        .addMiddleware(middleware)
        .read(contactsJoinQuery()),
    ),
  );
  const enriched = await collectAsync(
    contactsServiceOf(connection, verbose ? middleware : undefined).read(),
  );
  return { eager, enriched };
};
