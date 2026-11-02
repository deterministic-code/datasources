import { RepositoryFactory } from "@deterministic-code/datasources";
import { AddressRepository } from "./addressRepository";
import { AddressAttributeRepository } from "./addressAttributeRepository";
import { AddressLabelRepository } from "./addressLabelRepository";
import { AddressTypeRepository } from "./addressTypeRepository";
import { ContactRepository } from "./contactRepository";
import { ContactAttributeRepository } from "./contactAttributeRepository";
import { ContactLabelRepository } from "./contactLabelRepository";
import { EmailRepository } from "./emailRepository";
import { EmailAttributeRepository } from "./emailAttributeRepository";
import { EmailLabelRepository } from "./emailLabelRepository";
import { EmailTypeRepository } from "./emailTypeRepository";
import { GenderRepository } from "./genderRepository";
import { LabelRepository } from "./labelRepository";
import { PhoneRepository } from "./phoneRepository";
import { PhoneAttributeRepository } from "./phoneAttributeRepository";
import { PhoneLabelRepository } from "./phoneLabelRepository";
import { PhoneTypeRepository } from "./phoneTypeRepository";
export { address } from "../types/generated/views/address";
export { addressAttribute } from "../types/generated/datasource/addressAttribute";
export { addressLabel } from "../types/generated/datasource/addressLabel";
export { addressType } from "../types/generated/datasource/addressType";
export { contact } from "../types/generated/views/contact";
export { contactAttribute } from "../types/generated/datasource/contactAttribute";
export { contactLabel } from "../types/generated/datasource/contactLabel";
export { email } from "../types/generated/views/email";
export { emailAttribute } from "../types/generated/datasource/emailAttribute";
export { emailLabel } from "../types/generated/datasource/emailLabel";
export { emailType } from "../types/generated/datasource/emailType";
export { gender } from "../types/generated/datasource/gender";
export { label } from "../types/generated/datasource/label";
export { phone } from "../types/generated/views/phone";
export { phoneAttribute } from "../types/generated/datasource/phoneAttribute";
export { phoneLabel } from "../types/generated/datasource/phoneLabel";
export { phoneType } from "../types/generated/datasource/phoneType";

export const register = (factory: RepositoryFactory): void => {
  factory.register("Address", "addresses", (connection) => new AddressRepository(connection), (connection) => AddressRepository.createReadonly(connection));
  factory.register("AddressAttribute", "addressAttributes", (connection) => new AddressAttributeRepository(connection), (connection) => AddressAttributeRepository.createReadonly(connection));
  factory.register("AddressLabel", "addressLabels", (connection) => new AddressLabelRepository(connection), (connection) => AddressLabelRepository.createReadonly(connection));
  factory.register("AddressType", "addressTypes", (connection) => new AddressTypeRepository(connection), (connection) => AddressTypeRepository.createReadonly(connection));
  factory.register("Contact", "contacts", (connection) => new ContactRepository(connection), (connection) => ContactRepository.createReadonly(connection));
  factory.register("ContactAttribute", "contactAttributes", (connection) => new ContactAttributeRepository(connection), (connection) => ContactAttributeRepository.createReadonly(connection));
  factory.register("ContactLabel", "contactLabels", (connection) => new ContactLabelRepository(connection), (connection) => ContactLabelRepository.createReadonly(connection));
  factory.register("Email", "emails", (connection) => new EmailRepository(connection), (connection) => EmailRepository.createReadonly(connection));
  factory.register("EmailAttribute", "emailAttributes", (connection) => new EmailAttributeRepository(connection), (connection) => EmailAttributeRepository.createReadonly(connection));
  factory.register("EmailLabel", "emailLabels", (connection) => new EmailLabelRepository(connection), (connection) => EmailLabelRepository.createReadonly(connection));
  factory.register("EmailType", "emailTypes", (connection) => new EmailTypeRepository(connection), (connection) => EmailTypeRepository.createReadonly(connection));
  factory.register("Gender", "genders", (connection) => new GenderRepository(connection), (connection) => GenderRepository.createReadonly(connection));
  factory.register("Label", "labels", (connection) => new LabelRepository(connection), (connection) => LabelRepository.createReadonly(connection));
  factory.register("Phone", "phones", (connection) => new PhoneRepository(connection), (connection) => PhoneRepository.createReadonly(connection));
  factory.register("PhoneAttribute", "phoneAttributes", (connection) => new PhoneAttributeRepository(connection), (connection) => PhoneAttributeRepository.createReadonly(connection));
  factory.register("PhoneLabel", "phoneLabels", (connection) => new PhoneLabelRepository(connection), (connection) => PhoneLabelRepository.createReadonly(connection));
  factory.register("PhoneType", "phoneTypes", (connection) => new PhoneTypeRepository(connection), (connection) => PhoneTypeRepository.createReadonly(connection));
};
RepositoryFactory.bind(register);

declare module "@deterministic-code/datasources" {
  interface Datasource {
    readonly addresses: AddressRepository;
    readonly addressAttributes: AddressAttributeRepository;
    readonly addressLabels: AddressLabelRepository;
    readonly addressTypes: AddressTypeRepository;
    readonly contacts: ContactRepository;
    readonly contactAttributes: ContactAttributeRepository;
    readonly contactLabels: ContactLabelRepository;
    readonly emails: EmailRepository;
    readonly emailAttributes: EmailAttributeRepository;
    readonly emailLabels: EmailLabelRepository;
    readonly emailTypes: EmailTypeRepository;
    readonly genders: GenderRepository;
    readonly labels: LabelRepository;
    readonly phones: PhoneRepository;
    readonly phoneAttributes: PhoneAttributeRepository;
    readonly phoneLabels: PhoneLabelRepository;
    readonly phoneTypes: PhoneTypeRepository;
  }
}
