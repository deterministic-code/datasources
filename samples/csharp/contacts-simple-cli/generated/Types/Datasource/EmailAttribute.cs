// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type EmailAttribute. */
public class EmailAttribute : Base
{
    public long EmailId { get; set; }
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
}
