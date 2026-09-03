namespace Datasources.Tests.Contact;

internal class ContactBase : Base
{
    public int contact_source_id { get; set; }
    public string first_name { get; set; } = "";
    public string last_name { get; set; } = "";
    public string? email { get; set; }
    public string? notes { get; set; }
}
