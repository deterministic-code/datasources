// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type AddressBase. */
public class AddressBase : Base
{
    public long ContactId { get; set; }
    public long AddressTypeId { get; set; }
    public string DisplayName { get; set; } = "";
    public string Line1 { get; set; } = "";
    public string Line2 { get; set; } = "";
    public string Line3 { get; set; } = "";
    public string Neighborhood { get; set; } = "";
    public string District { get; set; } = "";
    public string City { get; set; } = "";
    public string StateProvince { get; set; } = "";
    public string PostalCode { get; set; } = "";
    public string CountryCode { get; set; } = "";
    public string FormattedAddress { get; set; } = "";
    public long? Latitude { get; set; }
    public long? Longitude { get; set; }
    public bool IsPrimary { get; set; }
}
