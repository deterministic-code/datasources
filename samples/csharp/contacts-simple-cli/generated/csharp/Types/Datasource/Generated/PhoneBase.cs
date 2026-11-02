// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type PhoneBase. */
public class PhoneBase : Base
{
    public long PhoneTypeId { get; set; }
    public long ContactId { get; set; }
    public string Number { get; set; } = "";
    public string DisplayName { get; set; } = "";
    public string Extention { get; set; } = "";
    public bool IsPrimary { get; set; }
    public bool IsVerified { get; set; }
}
