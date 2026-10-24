import type { Contact } from "../types/generated/views/contact";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";
import { AddressRepository } from "./address-repository";
import { PhoneRepository } from "./phone-repository";
import { ContactSettingsRepository } from "./contact-settings-repository";
import { ContactPreferencesRepository } from "./contact-preferences-repository";
import { ContactSourceRepository } from "./contact-source-repository";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Contact>()
    .add({ entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], createProperties: ["uuid", "version", "contactSourceId", "firstName", "lastName", "email", "notes"], updateProperties: ["uuid", "version", "contactSourceId", "firstName", "lastName", "email", "notes"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .add("mysql", { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], createProperties: ["contactSourceId", "firstName", "lastName", "email", "notes"], updateProperties: ["uuid", "version", "contactSourceId", "firstName", "lastName", "email", "notes"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .add("postgres", { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), ids: ["id"], createProperties: ["contactSourceId", "firstName", "lastName", "email", "notes"], updateProperties: ["uuid", "version", "contactSourceId", "firstName", "lastName", "email", "notes"], readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<Contact>()
    .add({ entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .add("mysql", { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .add("postgres", { entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["contactSourceId", "contact_source_id"], ["firstName", "first_name"], ["lastName", "last_name"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactSourceId", "firstName", "lastName", "email", "notes"] })
    .for(connection);

export class ContactRepository extends CRUDRepository<Contact> {
  readonly address: AddressRepository;
  readonly phone: PhoneRepository;
  readonly contactSettings: ContactSettingsRepository;
  readonly contactPreferences: ContactPreferencesRepository;
  readonly contactSource: ContactSourceRepository;
  constructor(connection: IConnection) {
    super(connection, options(connection));
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
    const repo = new ReadonlyRepository<Contact>(connection, readonlyOptions(connection));
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
