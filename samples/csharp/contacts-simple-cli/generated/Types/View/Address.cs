// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View Address. */
public class Address
{
    public long Id { get; set; }
    public System.Guid Uuid { get; set; } = Guid.NewGuid();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
    public long ContactId { get; set; }
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
    public string AddressTypeName { get; set; } = "";
    public string Description { get; set; } = "";
    public Dictionary<string, string> Attributes { get; set; } = [];
    public List<Backend.Types.Datasource.Label> Labels { get; set; } = [];
}
