namespace Datasources.Tests.Contact;

internal class Base
{
    public int Id { get; set; }
    public Guid Uuid { get; set; }
    public DateTime Created { get; set; }
    public DateTime Updated { get; set; }
    public byte[] Version { get; set; } = [];
}
