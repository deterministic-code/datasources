import { RepositoryFactory } from "@deterministic-code/datasources";
import { AddressesRepository } from "./addressesRepository";
import { AddressTypesRepository } from "./addressTypesRepository";
import { ContactsRepository } from "./contactsRepository";
import { ContactCardsRepository } from "./contactCardsRepository";
import { ContactChangeLogsRepository } from "./contactChangeLogsRepository";
import { ContactGroupsRepository } from "./contactGroupsRepository";
import { ContactGroupMembersRepository } from "./contactGroupMembersRepository";
import { ContactPreferencesRepository } from "./contactPreferencesRepository";
import { ContactSettingsRepository } from "./contactSettingsRepository";
import { ContactSourcesRepository } from "./contactSourcesRepository";
import { LegacyContactsRepository } from "./legacyContactsRepository";
import { LocalePreferencesRepository } from "./localePreferencesRepository";
import { PhonesRepository } from "./phonesRepository";
import { PhoneTypesRepository } from "./phoneTypesRepository";
export { address } from "../types/view/address";
export { addressType } from "../types/datasource/addressType";
export { contact } from "../types/view/contact";
export { contactCard } from "../types/view/contactCard";
export { contactChangeLog } from "../types/view/contactChangeLog";
export { contactGroup } from "../types/view/contactGroup";
export { contactGroupMember } from "../types/datasource/contactGroupMember";
export { contactPreference } from "../types/datasource/contactPreference";
export { contactSetting } from "../types/datasource/contactSetting";
export { contactSource } from "../types/datasource/contactSource";
export { legacyContact } from "../types/view/legacyContact";
export { localePreference } from "../types/view/localePreference";
export { phone } from "../types/view/phone";
export { phoneType } from "../types/datasource/phoneType";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Address", "addresses", (connection) => new AddressesRepository(connection), (connection) => AddressesRepository.createReadonly(connection));
  factory.register("AddressType", "addressTypes", (connection) => new AddressTypesRepository(connection), (connection) => AddressTypesRepository.createReadonly(connection));
  factory.register("Contact", "contacts", (connection) => new ContactsRepository(connection), (connection) => ContactsRepository.createReadonly(connection));
  factory.register("ContactCard", "contactCards", (connection) => new ContactCardsRepository(connection), (connection) => ContactCardsRepository.createReadonly(connection));
  factory.register("ContactChangeLog", "contactChangeLogs", (connection) => new ContactChangeLogsRepository(connection), (connection) => ContactChangeLogsRepository.createReadonly(connection));
  factory.register("ContactGroup", "contactGroups", (connection) => new ContactGroupsRepository(connection), (connection) => ContactGroupsRepository.createReadonly(connection));
  factory.register("ContactGroupMember", "contactGroupMembers", (connection) => new ContactGroupMembersRepository(connection), (connection) => ContactGroupMembersRepository.createReadonly(connection));
  factory.register("ContactPreference", "contactPreferences", (connection) => new ContactPreferencesRepository(connection), (connection) => ContactPreferencesRepository.createReadonly(connection));
  factory.register("ContactSetting", "contactSettings", (connection) => new ContactSettingsRepository(connection), (connection) => ContactSettingsRepository.createReadonly(connection));
  factory.register("ContactSource", "contactSources", (connection) => new ContactSourcesRepository(connection), (connection) => ContactSourcesRepository.createReadonly(connection));
  factory.register("LegacyContact", "legacyContacts", (connection) => new LegacyContactsRepository(connection), (connection) => LegacyContactsRepository.createReadonly(connection));
  factory.register("LocalePreference", "localePreferences", (connection) => new LocalePreferencesRepository(connection), (connection) => LocalePreferencesRepository.createReadonly(connection));
  factory.register("Phone", "phones", (connection) => new PhonesRepository(connection), (connection) => PhonesRepository.createReadonly(connection));
  factory.register("PhoneType", "phoneTypes", (connection) => new PhoneTypesRepository(connection), (connection) => PhoneTypesRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly addresses: AddressesRepository;
    readonly addressTypes: AddressTypesRepository;
    readonly contacts: ContactsRepository;
    readonly contactCards: ContactCardsRepository;
    readonly contactChangeLogs: ContactChangeLogsRepository;
    readonly contactGroups: ContactGroupsRepository;
    readonly contactGroupMembers: ContactGroupMembersRepository;
    readonly contactPreferences: ContactPreferencesRepository;
    readonly contactSettings: ContactSettingsRepository;
    readonly contactSources: ContactSourcesRepository;
    readonly legacyContacts: LegacyContactsRepository;
    readonly localePreferences: LocalePreferencesRepository;
    readonly phones: PhonesRepository;
    readonly phoneTypes: PhoneTypesRepository;
  }
}
