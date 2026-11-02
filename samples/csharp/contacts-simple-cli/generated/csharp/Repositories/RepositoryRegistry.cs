using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Address", "Addresses", connection => new AddressRepository(connection), connection => AddressRepository.CreateReadonly(connection));
        factory.Register("AddressAttribute", "AddressAttributes", connection => new AddressAttributeRepository(connection), connection => AddressAttributeRepository.CreateReadonly(connection));
        factory.Register("AddressLabel", "AddressLabels", connection => new AddressLabelRepository(connection), connection => AddressLabelRepository.CreateReadonly(connection));
        factory.Register("AddressType", "AddressTypes", connection => new AddressTypeRepository(connection), connection => AddressTypeRepository.CreateReadonly(connection));
        factory.Register("Contact", "Contacts", connection => new ContactRepository(connection), connection => ContactRepository.CreateReadonly(connection));
        factory.Register("ContactAttribute", "ContactAttributes", connection => new ContactAttributeRepository(connection), connection => ContactAttributeRepository.CreateReadonly(connection));
        factory.Register("ContactLabel", "ContactLabels", connection => new ContactLabelRepository(connection), connection => ContactLabelRepository.CreateReadonly(connection));
        factory.Register("Email", "Emails", connection => new EmailRepository(connection), connection => EmailRepository.CreateReadonly(connection));
        factory.Register("EmailAttribute", "EmailAttributes", connection => new EmailAttributeRepository(connection), connection => EmailAttributeRepository.CreateReadonly(connection));
        factory.Register("EmailLabel", "EmailLabels", connection => new EmailLabelRepository(connection), connection => EmailLabelRepository.CreateReadonly(connection));
        factory.Register("EmailType", "EmailTypes", connection => new EmailTypeRepository(connection), connection => EmailTypeRepository.CreateReadonly(connection));
        factory.Register("Gender", "Genders", connection => new GenderRepository(connection), connection => GenderRepository.CreateReadonly(connection));
        factory.Register("Label", "Labels", connection => new LabelRepository(connection), connection => LabelRepository.CreateReadonly(connection));
        factory.Register("Phone", "Phones", connection => new PhoneRepository(connection), connection => PhoneRepository.CreateReadonly(connection));
        factory.Register("PhoneAttribute", "PhoneAttributes", connection => new PhoneAttributeRepository(connection), connection => PhoneAttributeRepository.CreateReadonly(connection));
        factory.Register("PhoneLabel", "PhoneLabels", connection => new PhoneLabelRepository(connection), connection => PhoneLabelRepository.CreateReadonly(connection));
        factory.Register("PhoneType", "PhoneTypes", connection => new PhoneTypeRepository(connection), connection => PhoneTypeRepository.CreateReadonly(connection));
    }
}
