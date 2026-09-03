namespace Datasources.Tests.Contact;

internal sealed class ContactChangeLog : Base
{
    public DateTime occurred_at { get; set; }
    public int contact_id { get; set; }
    public string changed_by { get; set; } = "";
    public string change_summary { get; set; } = "";
}
