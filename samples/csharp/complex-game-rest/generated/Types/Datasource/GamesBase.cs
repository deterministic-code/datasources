// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type GamesBase. */
public class GamesBase : Base
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Location { get; set; } = "";
    public bool IsEnabled { get; set; } = true;
    public bool IsDeleted { get; set; }
}
