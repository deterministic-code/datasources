namespace Datasources.Tests.Contact;

internal sealed class Phone : Base
{
    public Contact Contact { get; set; } = new();
    public PhoneType PhoneType { get; set; } = new();
    public string Number { get; set; } = "";
}
