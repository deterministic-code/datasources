import type { RepositoryFactory } from "@datasources/core";
import { AddressRepository } from "./address-repository.ts";
import { AddressTypeRepository } from "./address-type-repository.ts";
import { ContactRepository } from "./contact-repository.ts";
import { ContactChangeLogRepository } from "./contact-change-log-repository.ts";
import { ContactGroupRepository } from "./contact-group-repository.ts";
import { ContactGroupMemberRepository } from "./contact-group-member-repository.ts";
import { ContactPreferencesRepository } from "./contact-preferences-repository.ts";
import { ContactSettingsRepository } from "./contact-settings-repository.ts";
import { ContactSourceRepository } from "./contact-source-repository.ts";
import { LegacyContactRepository } from "./legacy-contact-repository.ts";
import { PhoneRepository } from "./phone-repository.ts";
import { PhoneTypeRepository } from "./phone-type-repository.ts";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Address", "addresses", (connection) => new AddressRepository(connection), (connection) => AddressRepository.createReadonly(connection));
  factory.register("AddressType", "addressTypes", (connection) => new AddressTypeRepository(connection), (connection) => AddressTypeRepository.createReadonly(connection));
  factory.register("Contact", "contacts", (connection) => new ContactRepository(connection), (connection) => ContactRepository.createReadonly(connection));
  factory.register("ContactChangeLog", "contactChangeLogs", (connection) => new ContactChangeLogRepository(connection), (connection) => ContactChangeLogRepository.createReadonly(connection));
  factory.register("ContactGroup", "contactGroups", (connection) => new ContactGroupRepository(connection), (connection) => ContactGroupRepository.createReadonly(connection));
  factory.register("ContactGroupMember", "contactGroupMembers", (connection) => new ContactGroupMemberRepository(connection), (connection) => ContactGroupMemberRepository.createReadonly(connection));
  factory.register("ContactPreferences", "contactPreferences", (connection) => new ContactPreferencesRepository(connection), (connection) => ContactPreferencesRepository.createReadonly(connection));
  factory.register("ContactSettings", "contactSettings", (connection) => new ContactSettingsRepository(connection), (connection) => ContactSettingsRepository.createReadonly(connection));
  factory.register("ContactSource", "contactSources", (connection) => new ContactSourceRepository(connection), (connection) => ContactSourceRepository.createReadonly(connection));
  factory.register("LegacyContact", "legacyContacts", (connection) => new LegacyContactRepository(connection), (connection) => LegacyContactRepository.createReadonly(connection));
  factory.register("Phone", "phones", (connection) => new PhoneRepository(connection), (connection) => PhoneRepository.createReadonly(connection));
  factory.register("PhoneType", "phoneTypes", (connection) => new PhoneTypeRepository(connection), (connection) => PhoneTypeRepository.createReadonly(connection));
};

declare module "@datasources/core" {
  interface Datasource {
    readonly addresses: AddressRepository;
    readonly addressTypes: AddressTypeRepository;
    readonly contacts: ContactRepository;
    readonly contactChangeLogs: ContactChangeLogRepository;
    readonly contactGroups: ContactGroupRepository;
    readonly contactGroupMembers: ContactGroupMemberRepository;
    readonly contactPreferences: ContactPreferencesRepository;
    readonly contactSettings: ContactSettingsRepository;
    readonly contactSources: ContactSourceRepository;
    readonly legacyContacts: LegacyContactRepository;
    readonly phones: PhoneRepository;
    readonly phoneTypes: PhoneTypeRepository;
  }
}
