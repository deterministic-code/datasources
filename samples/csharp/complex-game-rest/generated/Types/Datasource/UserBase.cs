// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type UserBase. */
public class UserBase : Base
{
    public string Username { get; set; } = "";
    public string Password { get; set; } = "";
    public bool IsEnabled { get; set; } = true;
}
