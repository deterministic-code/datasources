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
    super(connection, { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] });
    this.address = new AddressRepository(connection);
    this.phone = new PhoneRepository(connection);
    this.contactSource = new ContactSourceRepository(connection);
    this.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.address);
    this.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.phone);
    this.addEnriched({ destinationProperty: "contactSourceName", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.contactSource);
  }
}
