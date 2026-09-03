namespace Datasources.Tests.Contact;

internal sealed class Phone : Base
{
    public int contact_id { get; set; }
    public string number { get; set; } = "";
    public string? label { get; set; }
}
