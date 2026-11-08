// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactSetting. */
public class ContactSetting : Base
{
    public long ContactId { get; set; }
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
}
