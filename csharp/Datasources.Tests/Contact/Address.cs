namespace Datasources.Tests.Contact;

internal sealed class Address : Base
{
    public int contact_id { get; set; }
    public string line1 { get; set; } = "";
    public string? line2 { get; set; }
    public string city { get; set; } = "";
    public string? region { get; set; }
    public string? postal_code { get; set; }
    public string? country { get; set; }
}
