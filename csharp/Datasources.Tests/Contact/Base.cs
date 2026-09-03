namespace Datasources.Tests.Contact;

internal class Base
{
    public int id { get; set; }
    public Guid uuid { get; set; }
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    public byte[] version { get; set; } = [];
}
