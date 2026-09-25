namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactsView : Contact
{
    public string contact_source { get; set; } = "";
    public string contact_type { get; set; } = "";
    public List<Address> addresses { get; set; } = [];
    public List<Phone> phones { get; set; } = [];
    public List<ContactGroup> groups { get; set; } = [];
}
