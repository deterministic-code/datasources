use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Address;
use super::super::AddressAttribute;
use super::super::AddressLabel;
use super::super::AddressType;
use super::super::Contact;
use super::super::ContactAttribute;
use super::super::ContactLabel;
use super::super::Email;
use super::super::EmailAttribute;
use super::super::EmailLabel;
use super::super::EmailType;
use super::super::Gender;
use super::super::Label;
use super::super::Phone;
use super::super::PhoneAttribute;
use super::super::PhoneLabel;
use super::super::PhoneType;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Address",
        "addresses",
        |connection| Arc::new(super::addressesRepository::AddressesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Address>>,
        |connection| Arc::new(super::addressesRepository::AddressesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Address>>,
    );
    factory.register_named(
        "AddressAttribute",
        "address_attributes",
        |connection| Arc::new(super::addressAttributesRepository::AddressAttributesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressAttribute>>,
        |connection| Arc::new(super::addressAttributesRepository::AddressAttributesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressAttribute>>,
    );
    factory.register_named(
        "AddressLabel",
        "address_labels",
        |connection| Arc::new(super::addressLabelsRepository::AddressLabelsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressLabel>>,
        |connection| Arc::new(super::addressLabelsRepository::AddressLabelsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressLabel>>,
    );
    factory.register_named(
        "AddressType",
        "address_types",
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressType>>,
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressType>>,
    );
    factory.register_named(
        "Contact",
        "contacts",
        |connection| Arc::new(super::contactsRepository::ContactsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Contact>>,
        |connection| Arc::new(super::contactsRepository::ContactsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Contact>>,
    );
    factory.register_named(
        "ContactAttribute",
        "contact_attributes",
        |connection| Arc::new(super::contactAttributesRepository::ContactAttributesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactAttribute>>,
        |connection| Arc::new(super::contactAttributesRepository::ContactAttributesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactAttribute>>,
    );
    factory.register_named(
        "ContactLabel",
        "contact_labels",
        |connection| Arc::new(super::contactLabelsRepository::ContactLabelsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactLabel>>,
        |connection| Arc::new(super::contactLabelsRepository::ContactLabelsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactLabel>>,
    );
    factory.register_named(
        "Email",
        "emails",
        |connection| Arc::new(super::emailsRepository::EmailsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Email>>,
        |connection| Arc::new(super::emailsRepository::EmailsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Email>>,
    );
    factory.register_named(
        "EmailAttribute",
        "email_attributes",
        |connection| Arc::new(super::emailAttributesRepository::EmailAttributesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<EmailAttribute>>,
        |connection| Arc::new(super::emailAttributesRepository::EmailAttributesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<EmailAttribute>>,
    );
    factory.register_named(
        "EmailLabel",
        "email_labels",
        |connection| Arc::new(super::emailLabelsRepository::EmailLabelsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<EmailLabel>>,
        |connection| Arc::new(super::emailLabelsRepository::EmailLabelsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<EmailLabel>>,
    );
    factory.register_named(
        "EmailType",
        "email_types",
        |connection| Arc::new(super::emailTypesRepository::EmailTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<EmailType>>,
        |connection| Arc::new(super::emailTypesRepository::EmailTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<EmailType>>,
    );
    factory.register_named(
        "Gender",
        "genders",
        |connection| Arc::new(super::gendersRepository::GendersRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Gender>>,
        |connection| Arc::new(super::gendersRepository::GendersRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Gender>>,
    );
    factory.register_named(
        "Label",
        "labels",
        |connection| Arc::new(super::labelsRepository::LabelsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Label>>,
        |connection| Arc::new(super::labelsRepository::LabelsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Label>>,
    );
    factory.register_named(
        "Phone",
        "phones",
        |connection| Arc::new(super::phonesRepository::PhonesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Phone>>,
        |connection| Arc::new(super::phonesRepository::PhonesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Phone>>,
    );
    factory.register_named(
        "PhoneAttribute",
        "phone_attributes",
        |connection| Arc::new(super::phoneAttributesRepository::PhoneAttributesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneAttribute>>,
        |connection| Arc::new(super::phoneAttributesRepository::PhoneAttributesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneAttribute>>,
    );
    factory.register_named(
        "PhoneLabel",
        "phone_labels",
        |connection| Arc::new(super::phoneLabelsRepository::PhoneLabelsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneLabel>>,
        |connection| Arc::new(super::phoneLabelsRepository::PhoneLabelsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneLabel>>,
    );
    factory.register_named(
        "PhoneType",
        "phone_types",
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneType>>,
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneType>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository>;
    fn address_attributes(&self) -> Arc<super::addressAttributesRepository::AddressAttributesRepository>;
    fn address_labels(&self) -> Arc<super::addressLabelsRepository::AddressLabelsRepository>;
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository>;
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository>;
    fn contact_attributes(&self) -> Arc<super::contactAttributesRepository::ContactAttributesRepository>;
    fn contact_labels(&self) -> Arc<super::contactLabelsRepository::ContactLabelsRepository>;
    fn emails(&self) -> Arc<super::emailsRepository::EmailsRepository>;
    fn email_attributes(&self) -> Arc<super::emailAttributesRepository::EmailAttributesRepository>;
    fn email_labels(&self) -> Arc<super::emailLabelsRepository::EmailLabelsRepository>;
    fn email_types(&self) -> Arc<super::emailTypesRepository::EmailTypesRepository>;
    fn genders(&self) -> Arc<super::gendersRepository::GendersRepository>;
    fn labels(&self) -> Arc<super::labelsRepository::LabelsRepository>;
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository>;
    fn phone_attributes(&self) -> Arc<super::phoneAttributesRepository::PhoneAttributesRepository>;
    fn phone_labels(&self) -> Arc<super::phoneLabelsRepository::PhoneLabelsRepository>;
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository>;
}

impl Repositories for Arc<dyn Datasource> {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository> {
        Arc::new(super::addressesRepository::AddressesRepository::new(self.bind_connection(None)).expect("addresses"))
    }
    fn address_attributes(&self) -> Arc<super::addressAttributesRepository::AddressAttributesRepository> {
        Arc::new(super::addressAttributesRepository::AddressAttributesRepository::new(self.bind_connection(None)).expect("address_attributes"))
    }
    fn address_labels(&self) -> Arc<super::addressLabelsRepository::AddressLabelsRepository> {
        Arc::new(super::addressLabelsRepository::AddressLabelsRepository::new(self.bind_connection(None)).expect("address_labels"))
    }
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository> {
        Arc::new(super::addressTypesRepository::AddressTypesRepository::new(self.bind_connection(None)).expect("address_types"))
    }
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository> {
        Arc::new(super::contactsRepository::ContactsRepository::new(self.bind_connection(None)).expect("contacts"))
    }
    fn contact_attributes(&self) -> Arc<super::contactAttributesRepository::ContactAttributesRepository> {
        Arc::new(super::contactAttributesRepository::ContactAttributesRepository::new(self.bind_connection(None)).expect("contact_attributes"))
    }
    fn contact_labels(&self) -> Arc<super::contactLabelsRepository::ContactLabelsRepository> {
        Arc::new(super::contactLabelsRepository::ContactLabelsRepository::new(self.bind_connection(None)).expect("contact_labels"))
    }
    fn emails(&self) -> Arc<super::emailsRepository::EmailsRepository> {
        Arc::new(super::emailsRepository::EmailsRepository::new(self.bind_connection(None)).expect("emails"))
    }
    fn email_attributes(&self) -> Arc<super::emailAttributesRepository::EmailAttributesRepository> {
        Arc::new(super::emailAttributesRepository::EmailAttributesRepository::new(self.bind_connection(None)).expect("email_attributes"))
    }
    fn email_labels(&self) -> Arc<super::emailLabelsRepository::EmailLabelsRepository> {
        Arc::new(super::emailLabelsRepository::EmailLabelsRepository::new(self.bind_connection(None)).expect("email_labels"))
    }
    fn email_types(&self) -> Arc<super::emailTypesRepository::EmailTypesRepository> {
        Arc::new(super::emailTypesRepository::EmailTypesRepository::new(self.bind_connection(None)).expect("email_types"))
    }
    fn genders(&self) -> Arc<super::gendersRepository::GendersRepository> {
        Arc::new(super::gendersRepository::GendersRepository::new(self.bind_connection(None)).expect("genders"))
    }
    fn labels(&self) -> Arc<super::labelsRepository::LabelsRepository> {
        Arc::new(super::labelsRepository::LabelsRepository::new(self.bind_connection(None)).expect("labels"))
    }
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository> {
        Arc::new(super::phonesRepository::PhonesRepository::new(self.bind_connection(None)).expect("phones"))
    }
    fn phone_attributes(&self) -> Arc<super::phoneAttributesRepository::PhoneAttributesRepository> {
        Arc::new(super::phoneAttributesRepository::PhoneAttributesRepository::new(self.bind_connection(None)).expect("phone_attributes"))
    }
    fn phone_labels(&self) -> Arc<super::phoneLabelsRepository::PhoneLabelsRepository> {
        Arc::new(super::phoneLabelsRepository::PhoneLabelsRepository::new(self.bind_connection(None)).expect("phone_labels"))
    }
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository> {
        Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(self.bind_connection(None)).expect("phone_types"))
    }
}
