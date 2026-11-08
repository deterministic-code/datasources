// schema-version: 1.0
namespace Backend.Types.View;

/** View Phone. */
public class Phone
{
    public long Id { get; set; }
    public string Uuid { get; set; } = Guid.NewGuid().ToString();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
    public string ContactId { get; set; } = "";
    public string Number { get; set; } = "";
    public string? Label { get; set; }
    public string PhoneTypeName { get; set; } = "";
    public string? Description { get; set; }
}
