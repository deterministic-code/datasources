import { RepositoryFactory } from "@deterministic-code/datasources";
import { AddressesRepository } from "./addressesRepository";
import { AddressAttributesRepository } from "./addressAttributesRepository";
import { AddressLabelsRepository } from "./addressLabelsRepository";
import { AddressTypesRepository } from "./addressTypesRepository";
import { ContactsRepository } from "./contactsRepository";
import { ContactAttributesRepository } from "./contactAttributesRepository";
import { ContactLabelsRepository } from "./contactLabelsRepository";
import { EmailsRepository } from "./emailsRepository";
import { EmailAttributesRepository } from "./emailAttributesRepository";
import { EmailLabelsRepository } from "./emailLabelsRepository";
import { EmailTypesRepository } from "./emailTypesRepository";
import { GendersRepository } from "./gendersRepository";
import { LabelsRepository } from "./labelsRepository";
import { PhonesRepository } from "./phonesRepository";
import { PhoneAttributesRepository } from "./phoneAttributesRepository";
import { PhoneLabelsRepository } from "./phoneLabelsRepository";
import { PhoneTypesRepository } from "./phoneTypesRepository";
export { address } from "../types/view/address";
export { addressAttribute } from "../types/datasource/addressAttribute";
export { addressLabel } from "../types/datasource/addressLabel";
export { addressType } from "../types/datasource/addressType";
export { contact } from "../types/view/contact";
export { contactAttribute } from "../types/datasource/contactAttribute";
export { contactLabel } from "../types/datasource/contactLabel";
export { email } from "../types/view/email";
export { emailAttribute } from "../types/datasource/emailAttribute";
export { emailLabel } from "../types/datasource/emailLabel";
export { emailType } from "../types/datasource/emailType";
export { gender } from "../types/datasource/gender";
export { label } from "../types/datasource/label";
export { phone } from "../types/view/phone";
export { phoneAttribute } from "../types/datasource/phoneAttribute";
export { phoneLabel } from "../types/datasource/phoneLabel";
export { phoneType } from "../types/datasource/phoneType";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Address", "addresses", (connection) => new AddressesRepository(connection), (connection) => AddressesRepository.createReadonly(connection));
  factory.register("AddressAttribute", "addressAttributes", (connection) => new AddressAttributesRepository(connection), (connection) => AddressAttributesRepository.createReadonly(connection));
  factory.register("AddressLabel", "addressLabels", (connection) => new AddressLabelsRepository(connection), (connection) => AddressLabelsRepository.createReadonly(connection));
  factory.register("AddressType", "addressTypes", (connection) => new AddressTypesRepository(connection), (connection) => AddressTypesRepository.createReadonly(connection));
  factory.register("Contact", "contacts", (connection) => new ContactsRepository(connection), (connection) => ContactsRepository.createReadonly(connection));
  factory.register("ContactAttribute", "contactAttributes", (connection) => new ContactAttributesRepository(connection), (connection) => ContactAttributesRepository.createReadonly(connection));
  factory.register("ContactLabel", "contactLabels", (connection) => new ContactLabelsRepository(connection), (connection) => ContactLabelsRepository.createReadonly(connection));
  factory.register("Email", "emails", (connection) => new EmailsRepository(connection), (connection) => EmailsRepository.createReadonly(connection));
  factory.register("EmailAttribute", "emailAttributes", (connection) => new EmailAttributesRepository(connection), (connection) => EmailAttributesRepository.createReadonly(connection));
  factory.register("EmailLabel", "emailLabels", (connection) => new EmailLabelsRepository(connection), (connection) => EmailLabelsRepository.createReadonly(connection));
  factory.register("EmailType", "emailTypes", (connection) => new EmailTypesRepository(connection), (connection) => EmailTypesRepository.createReadonly(connection));
  factory.register("Gender", "genders", (connection) => new GendersRepository(connection), (connection) => GendersRepository.createReadonly(connection));
  factory.register("Label", "labels", (connection) => new LabelsRepository(connection), (connection) => LabelsRepository.createReadonly(connection));
  factory.register("Phone", "phones", (connection) => new PhonesRepository(connection), (connection) => PhonesRepository.createReadonly(connection));
  factory.register("PhoneAttribute", "phoneAttributes", (connection) => new PhoneAttributesRepository(connection), (connection) => PhoneAttributesRepository.createReadonly(connection));
  factory.register("PhoneLabel", "phoneLabels", (connection) => new PhoneLabelsRepository(connection), (connection) => PhoneLabelsRepository.createReadonly(connection));
  factory.register("PhoneType", "phoneTypes", (connection) => new PhoneTypesRepository(connection), (connection) => PhoneTypesRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly addresses: AddressesRepository;
    readonly addressAttributes: AddressAttributesRepository;
    readonly addressLabels: AddressLabelsRepository;
    readonly addressTypes: AddressTypesRepository;
    readonly contacts: ContactsRepository;
    readonly contactAttributes: ContactAttributesRepository;
    readonly contactLabels: ContactLabelsRepository;
    readonly emails: EmailsRepository;
    readonly emailAttributes: EmailAttributesRepository;
    readonly emailLabels: EmailLabelsRepository;
    readonly emailTypes: EmailTypesRepository;
    readonly genders: GendersRepository;
    readonly labels: LabelsRepository;
    readonly phones: PhonesRepository;
    readonly phoneAttributes: PhoneAttributesRepository;
    readonly phoneLabels: PhoneLabelsRepository;
    readonly phoneTypes: PhoneTypesRepository;
  }
}
