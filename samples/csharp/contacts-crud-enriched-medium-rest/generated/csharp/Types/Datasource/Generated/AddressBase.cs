// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type AddressBase. */
public class AddressBase : Base
{
    public long? Latitude { get; set; }
    public string? LabelNote { get; set; }
    public uint? Floor { get; set; }
    public string? Score { get; set; }
    public string? ExternalId { get; set; }
    public bool? IsVerified { get; set; }
    public System.DateTime? VerifiedAt { get; set; }
    public byte[]? Stamp { get; set; }
    public long ContactId { get; set; }
    public long AddressTypeId { get; set; }
    public string Line1 { get; set; } = "";
    public string? Line2 { get; set; }
    public string City { get; set; } = "";
    public string? Region { get; set; }
    public string? PostalCode { get; set; }
    public string? Country { get; set; }
}
