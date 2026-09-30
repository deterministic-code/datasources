namespace Datasources.Tests.Contact;

internal sealed class Contact : ContactBase
{
    public ContactSource ContactSource { get; set; } = new();
    public ContactType ContactType { get; set; } = new();
    public List<Address> Addresses { get; set; } = [];
    public List<Phone> Phones { get; set; } = [];
}
