// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type GameState. */
public class GameState : Base
{
    public long GameId { get; set; }
    public long UserId { get; set; }
    public string State { get; set; } = "";
}
