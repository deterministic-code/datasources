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
    addresses.addEnriched({ destinationProperty: "AddressType", sourceProperty: "AddressType", relatedKey: "Id" }, addressTypes);
    phones.addEnriched({ destinationProperty: "PhoneType", sourceProperty: "PhoneType", relatedKey: "Id" }, phoneTypes);
    contacts.addEnriched({ destinationProperty: "ContactType", sourceProperty: "ContactType", relatedKey: "Id" }, types);
    contacts.addEnriched({ destinationProperty: "ContactSource", sourceProperty: "ContactSource", relatedKey: "Id" }, sources);
    contacts.addEnriched({ destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, addresses);
    contacts.addEnriched({ destinationProperty: "Phones", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, phones);
    return contacts;
  },
};
