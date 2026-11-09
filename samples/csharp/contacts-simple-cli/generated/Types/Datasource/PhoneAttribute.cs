// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type PhoneAttribute. */
public class PhoneAttribute : Base
{
    public long PhoneId { get; set; }
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
}
