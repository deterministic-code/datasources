import type { ICRUDRepository } from "../../ICRUDRepository.ts";
import type { IEnriched } from "../../IEnriched.ts";
import type { IReadonlyRepository } from "../../IReadonlyRepository.ts";
import type { Address } from "./Address.ts";
import type { AddressType } from "./AddressType.ts";
import type { Contact } from "./Contact.ts";
import type { ContactSource } from "./ContactSource.ts";
import type { ContactType } from "./ContactType.ts";
import type { Phone } from "./Phone.ts";
import type { PhoneType } from "./PhoneType.ts";

type EnrichedCRUD<T> = ICRUDRepository<T> & IEnriched<T>;

export const ContactsService = {
  open(
    contacts: EnrichedCRUD<Contact>,
    types: IReadonlyRepository<ContactType>,
    sources: IReadonlyRepository<ContactSource>,
    addressTypes: IReadonlyRepository<AddressType>,
    phoneTypes: IReadonlyRepository<PhoneType>,
    addresses: EnrichedCRUD<Address>,
    phones: EnrichedCRUD<Phone>,
  ): EnrichedCRUD<Contact> {
    addresses.addEnriched({ into: "address_type_name", from: "address_type_id", by: "id", select: "name" }, addressTypes);
    phones.addEnriched({ into: "phone_type_name", from: "phone_type_id", by: "id", select: "name" }, phoneTypes);
    contacts.addEnriched({ into: "contact_type_name", from: "contact_type_id", by: "id", select: "name" }, types);
    contacts.addEnriched({ into: "contact_source_name", from: "contact_source_id", by: "id", select: "name" }, sources);
    contacts.addEnriched({ into: "addresses", from: "id", by: "contact_id", many: true }, addresses);
    contacts.addEnriched({ into: "phones", from: "id", by: "contact_id", many: true }, phones);
    return contacts;
  },
};
