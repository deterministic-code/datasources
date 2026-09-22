import { EnrichedCRUDRepository } from "../../EnrichedCRUDRepository.ts";
import type { IReadonlyRepository } from "../../IReadonlyRepository.ts";
import type { ICRUDRepository } from "../../ICRUDRepository.ts";
import type { Address } from "./Address.ts";
import type { AddressType } from "./AddressType.ts";
import type { Contact } from "./Contact.ts";
import type { ContactSource } from "./ContactSource.ts";
import type { ContactType } from "./ContactType.ts";
import type { Phone } from "./Phone.ts";
import type { PhoneType } from "./PhoneType.ts";

export class ContactsService extends EnrichedCRUDRepository<Contact> {
  constructor(
    contacts: ICRUDRepository<Contact>,
    types: IReadonlyRepository<ContactType>,
    sources: IReadonlyRepository<ContactSource>,
    addressTypes: IReadonlyRepository<AddressType>,
    phoneTypes: IReadonlyRepository<PhoneType>,
    addresses: ICRUDRepository<Address>,
    phones: ICRUDRepository<Phone>,
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
    this.addEnriched("addresses", "id", addresses, "contact_id", undefined, true, (node) =>
      node.include(addressTypes, "address_type_name", "address_type_id", "id", "name"),
    );
    this.addEnriched("phones", "id", phones, "contact_id", undefined, true, (node) =>
      node.include(phoneTypes, "phone_type_name", "phone_type_id", "id", "name"),
    );
  }
}
