namespace Datasources.Tests.Contact;

internal sealed class Contact : ContactBase
{
    public string contact_source_name { get; set; } = "";
    public string contact_type_name { get; set; } = "";
    public List<Address> addresses { get; set; } = [];
    public List<Phone> phones { get; set; } = [];
}
