// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactSource. */
public class ContactSource : Base
{
    public string Name { get; set; } = "";
    public string? Description { get; set; }
}
