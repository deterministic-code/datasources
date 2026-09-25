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
    addresses.addEnriched("address_type_name", "address_type_id", addressTypes, "id", "name");
    phones.addEnriched("phone_type_name", "phone_type_id", phoneTypes, "id", "name");
    contacts.addEnriched("contact_type_name", "contact_type_id", types, "id", "name");
    contacts.addEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
    contacts.addEnriched("addresses", "id", addresses, "contact_id", undefined, true);
    contacts.addEnriched("phones", "id", phones, "contact_id", undefined, true);
    return contacts;
  },
};
