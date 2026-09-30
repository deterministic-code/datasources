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
    addresses.addEnriched({ into: "AddressType", from: "AddressType", by: "Id" }, addressTypes);
    phones.addEnriched({ into: "PhoneType", from: "PhoneType", by: "Id" }, phoneTypes);
    contacts.addEnriched({ into: "ContactType", from: "ContactType", by: "Id" }, types);
    contacts.addEnriched({ into: "ContactSource", from: "ContactSource", by: "Id" }, sources);
    contacts.addEnriched({ into: "Addresses", from: "Id", by: "Contact", many: true }, addresses);
    contacts.addEnriched({ into: "Phones", from: "Id", by: "Contact", many: true }, phones);
    return contacts;
  },
};
