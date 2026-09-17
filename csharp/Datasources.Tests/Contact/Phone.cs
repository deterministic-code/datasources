namespace Datasources.Tests.Contact;

internal sealed class Phone : Base
{
    public int contact_id { get; set; }
    public int phone_type_id { get; set; }
    public string phone_type_name { get; set; } = "";
    public string number { get; set; } = "";
}
