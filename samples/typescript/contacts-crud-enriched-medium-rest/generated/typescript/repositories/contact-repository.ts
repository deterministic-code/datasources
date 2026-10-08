import type { Contact } from "../types/generated/views/contact.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@datasources/core";
import { AddressRepository } from "./address-repository.ts";
import { PhoneRepository } from "./phone-repository.ts";
import { ContactSettingsRepository } from "./contact-settings-repository.ts";
import { ContactPreferencesRepository } from "./contact-preferences-repository.ts";
import { ContactSourceRepository } from "./contact-source-repository.ts";

export class ContactRepository extends CRUDRepository<Contact> {
  readonly address: AddressRepository;
  readonly phone: PhoneRepository;
  readonly contactSettings: ContactSettingsRepository;
  readonly contactPreferences: ContactPreferencesRepository;
  readonly contactSource: ContactSourceRepository;
  constructor(connection: IConnection) {
    super(connection, { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] });
    this.address = new AddressRepository(connection);
    this.phone = new PhoneRepository(connection);
    this.contactSettings = new ContactSettingsRepository(connection);
    this.contactPreferences = new ContactPreferencesRepository(connection);
    this.contactSource = new ContactSourceRepository(connection);
    this.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.address);
    this.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, this.phone);
    this.addEnriched({ destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, this.contactSettings);
    this.addEnriched({ destinationProperty: "prefs", sourceProperty: "id", relatedKey: "contactId", type: "dictionary" }, this.contactPreferences);
    this.addEnriched({ destinationProperty: "contactSourceName", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "name", type: "object" }, this.contactSource);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<Contact>(connection, { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] });
    const address = new AddressRepository(connection);
    const phone = new PhoneRepository(connection);
    const contactSettings = new ContactSettingsRepository(connection);
    const contactPreferences = new ContactPreferencesRepository(connection);
    const contactSource = new ContactSourceRepository(connection);
    repo.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, address);
    repo.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phone);
    repo.addEnriched({ destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" }, contactSettings);
    repo.addEnriched({ destinationProperty: "prefs", sourceProperty: "id", relatedKey: "contactId", type: "dictionary" }, contactPreferences);
    repo.addEnriched({ destinationProperty: "contactSourceName", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "name", type: "object" }, contactSource);
    return repo;
  }
}
