// schema-version: 1.0
namespace Backend.Types.View;

/** View Phone. */
public class Phone : Backend.Types.Datasource.PhoneBase
{
    public string PhoneTypeName { get; set; } = "";
    public string? Description { get; set; }
}
