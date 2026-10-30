// schema-version: 1.0
namespace Backend.Types.View;

/** View Address. */
public class Address
{
    public long Id { get; set; }
    public string Uuid { get; set; } = Guid.NewGuid().ToString();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
    public long? Latitude { get; set; }
    public string? LabelNote { get; set; }
    public uint? Floor { get; set; }
    public string? Score { get; set; }
    public string? ExternalId { get; set; }
    public bool? IsVerified { get; set; }
    public System.DateTime? VerifiedAt { get; set; }
    public byte[]? Stamp { get; set; }
    public string ContactId { get; set; } = "";
    public string Line1 { get; set; } = "";
    public string? Line2 { get; set; }
    public string City { get; set; } = "";
    public string? Region { get; set; }
    public string? PostalCode { get; set; }
    public string? Country { get; set; }
    public string AddressTypeName { get; set; } = "";
    public string? Description { get; set; }
}
