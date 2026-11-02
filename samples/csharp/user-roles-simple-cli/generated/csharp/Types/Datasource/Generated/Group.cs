// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Group. */
public class Group : Base
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public bool IsEnabled { get; set; } = true;
}
