// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Role. */
public class Role : Base
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public bool IsEnabled { get; set; } = true;
}
