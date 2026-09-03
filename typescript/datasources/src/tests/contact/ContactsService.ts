import { DeepCRUDRepository } from "../../DeepCRUDRepository.ts";
import type { IReadonlyRepository } from "../../IReadonlyRepository.ts";
import type { IRepository } from "../../IRepository.ts";
import type { Address } from "./Address.ts";
import type { Contact } from "./Contact.ts";
import type { ContactSource } from "./ContactSource.ts";
import type { ContactType } from "./ContactType.ts";
import type { Phone } from "./Phone.ts";

export class ContactsService extends DeepCRUDRepository<Contact> {
  constructor(
    contacts: IRepository<Contact>,
    types: IReadonlyRepository<ContactType>,
    sources: IReadonlyRepository<ContactSource>,
    addresses: IRepository<Address>,
    phones: IRepository<Phone>,
  ) {
    super(contacts);
    this.addEnriched("contact_type_name", "contact_type_id", types, "id", "name");
    this.addEnriched(
      "contact_source_name",
      "contact_source_id",
      sources,
      "id",
      "name",
    );
    this.addEnriched("addresses", "id", addresses, "contact_id", undefined, true);
    this.addEnriched("phones", "id", phones, "contact_id", undefined, true);
  }
}
