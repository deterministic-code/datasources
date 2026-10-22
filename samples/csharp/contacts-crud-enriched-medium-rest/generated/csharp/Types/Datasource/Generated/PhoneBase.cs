// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type PhoneBase. */
public class PhoneBase : Base
{
    public long ContactId { get; set; }
    public long PhoneTypeId { get; set; }
    public string Number { get; set; } = "";
    public string? Label { get; set; }
}
