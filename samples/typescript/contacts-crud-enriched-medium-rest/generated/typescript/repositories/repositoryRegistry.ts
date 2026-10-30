import { RepositoryFactory } from "@deterministic-code/datasources";
import { AddressRepository } from "./addressRepository";
import { AddressTypeRepository } from "./addressTypeRepository";
import { ContactRepository } from "./contactRepository";
import { ContactCardRepository } from "./contactCardRepository";
import { ContactChangeLogRepository } from "./contactChangeLogRepository";
import { ContactGroupRepository } from "./contactGroupRepository";
import { ContactGroupMemberRepository } from "./contactGroupMemberRepository";
import { ContactPreferencesRepository } from "./contactPreferencesRepository";
import { ContactSettingsRepository } from "./contactSettingsRepository";
import { ContactSourceRepository } from "./contactSourceRepository";
import { LegacyContactRepository } from "./legacyContactRepository";
import { LocalePreferencesRepository } from "./localePreferencesRepository";
import { PhoneRepository } from "./phoneRepository";
import { PhoneTypeRepository } from "./phoneTypeRepository";
export { address } from "../types/generated/views/address";
export { addressType } from "../types/generated/datasource/addressType";
export { contact } from "../types/generated/views/contact";
export { contactCard } from "../types/generated/views/contactCard";
export { contactChangeLog } from "../types/generated/views/contactChangeLog";
export { contactGroup } from "../types/generated/views/contactGroup";
export { contactGroupMember } from "../types/generated/datasource/contactGroupMember";
export { contactPreferences } from "../types/generated/datasource/contactPreferences";
export { contactSettings } from "../types/generated/datasource/contactSettings";
export { contactSource } from "../types/generated/datasource/contactSource";
export { legacyContact } from "../types/generated/views/legacyContact";
export { localePreferences } from "../types/generated/views/localePreferences";
export { phone } from "../types/generated/views/phone";
export { phoneType } from "../types/generated/datasource/phoneType";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Address", "addresses", (connection) => new AddressRepository(connection), (connection) => AddressRepository.createReadonly(connection));
  factory.register("AddressType", "addressTypes", (connection) => new AddressTypeRepository(connection), (connection) => AddressTypeRepository.createReadonly(connection));
  factory.register("Contact", "contacts", (connection) => new ContactRepository(connection), (connection) => ContactRepository.createReadonly(connection));
  factory.register("ContactCard", "contactCards", (connection) => new ContactCardRepository(connection), (connection) => ContactCardRepository.createReadonly(connection));
  factory.register("ContactChangeLog", "contactChangeLogs", (connection) => new ContactChangeLogRepository(connection), (connection) => ContactChangeLogRepository.createReadonly(connection));
  factory.register("ContactGroup", "contactGroups", (connection) => new ContactGroupRepository(connection), (connection) => ContactGroupRepository.createReadonly(connection));
  factory.register("ContactGroupMember", "contactGroupMembers", (connection) => new ContactGroupMemberRepository(connection), (connection) => ContactGroupMemberRepository.createReadonly(connection));
  factory.register("ContactPreferences", "contactPreferences", (connection) => new ContactPreferencesRepository(connection), (connection) => ContactPreferencesRepository.createReadonly(connection));
  factory.register("ContactSettings", "contactSettings", (connection) => new ContactSettingsRepository(connection), (connection) => ContactSettingsRepository.createReadonly(connection));
  factory.register("ContactSource", "contactSources", (connection) => new ContactSourceRepository(connection), (connection) => ContactSourceRepository.createReadonly(connection));
  factory.register("LegacyContact", "legacyContacts", (connection) => new LegacyContactRepository(connection), (connection) => LegacyContactRepository.createReadonly(connection));
  factory.register("LocalePreferences", "localePreferences", (connection) => new LocalePreferencesRepository(connection), (connection) => LocalePreferencesRepository.createReadonly(connection));
  factory.register("Phone", "phones", (connection) => new PhoneRepository(connection), (connection) => PhoneRepository.createReadonly(connection));
  factory.register("PhoneType", "phoneTypes", (connection) => new PhoneTypeRepository(connection), (connection) => PhoneTypeRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly addresses: AddressRepository;
    readonly addressTypes: AddressTypeRepository;
    readonly contacts: ContactRepository;
    readonly contactCards: ContactCardRepository;
    readonly contactChangeLogs: ContactChangeLogRepository;
    readonly contactGroups: ContactGroupRepository;
    readonly contactGroupMembers: ContactGroupMemberRepository;
    readonly contactPreferences: ContactPreferencesRepository;
    readonly contactSettings: ContactSettingsRepository;
    readonly contactSources: ContactSourceRepository;
    readonly legacyContacts: LegacyContactRepository;
    readonly localePreferences: LocalePreferencesRepository;
    readonly phones: PhoneRepository;
    readonly phoneTypes: PhoneTypeRepository;
  }
}
