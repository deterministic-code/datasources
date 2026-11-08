// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type UserSetting. */
public class UserSetting : Base
{
    public long UserId { get; set; }
    public string Name { get; set; } = "";
    public string Value { get; set; } = "";
}
