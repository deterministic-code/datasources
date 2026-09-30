namespace Datasources.Tests.Contact;

internal sealed class Address : Base
{
    public Contact Contact { get; set; } = new();
    public AddressType AddressType { get; set; } = new();
    public string Line1 { get; set; } = "";
    public string? Line2 { get; set; }
    public string City { get; set; } = "";
    public string? Region { get; set; }
    public string? PostalCode { get; set; }
    public string? Country { get; set; }
}
