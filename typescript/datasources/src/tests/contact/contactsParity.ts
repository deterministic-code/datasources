import { collectAsync } from "../../collectAsync.ts";
import { ConsoleCommandMiddleware } from "../../ConsoleCommandMiddleware.ts";
import type { Cells } from "../../converters/Cell.ts";
import { FlatToTypeMapper } from "../../mappers/FlatToTypeMapper.ts";
import type { IConnection } from "../../IConnection.ts";
import { Repository } from "../../Repository.ts";
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
  const eager = new FlatToTypeMapper(contactsView).collapse(
    await collectAsync(
      new Repository<Cells>({ connection, entity: "contacts_base" })
        .use(middleware)
        .get(contactsJoinQuery()),
    ),
  );
  const enriched = await collectAsync(
    contactsServiceOf(connection, verbose ? middleware : undefined).get(),
  );
  return { eager, enriched };
};
