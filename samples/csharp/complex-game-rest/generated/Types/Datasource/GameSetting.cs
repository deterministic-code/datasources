// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type GameSetting. */
public class GameSetting : Base
{
    public long GameId { get; set; }
    public string Name { get; set; } = "";
    public string Value { get; set; } = "";
}
