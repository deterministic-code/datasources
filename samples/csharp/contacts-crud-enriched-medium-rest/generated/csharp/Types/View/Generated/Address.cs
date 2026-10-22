// schema-version: 1.0
namespace Backend.Types.View;

/** View Address. */
public class Address : Backend.Types.Datasource.AddressBase
{
    public string AddressTypeName { get; set; } = "";
    public string? Description { get; set; }
}
