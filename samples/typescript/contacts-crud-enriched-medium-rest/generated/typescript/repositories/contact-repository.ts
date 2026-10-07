import type { Contact } from "../types/generated/views/contact.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";
import { AddressRepository } from "./address-repository.ts";
import { PhoneRepository } from "./phone-repository.ts";
import { ContactSourceRepository } from "./contact-source-repository.ts";

export class ContactRepository extends CRUDRepository<Contact> {
  readonly address: AddressRepository;
  readonly phone: PhoneRepository;
  readonly contactSource: ContactSourceRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "contacts_base", entityMapper: new DictionaryMapper([["contacts_base", "contacts"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contact_source_id", "first_name", "last_name", "email", "notes"] });
    this.address = new AddressRepository(connection);
    this.phone = new PhoneRepository(connection);
    this.contactSource = new ContactSourceRepository(connection);
    this.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contact_id", type: "list" }, this.address);
    this.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contact_id", type: "list" }, this.phone);
    this.addEnriched({ destinationProperty: "contact_source_name", sourceProperty: "contact_source_id", relatedKey: "id", relatedProperty: "name", type: "object" }, this.contactSource);
  }
}
