// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Review. */
public class Review : Base
{
    public long GameId { get; set; }
    public long UserId { get; set; }
    public string Description { get; set; } = "";
    public decimal Rating { get; set; }
    public bool IsApproved { get; set; }
}
