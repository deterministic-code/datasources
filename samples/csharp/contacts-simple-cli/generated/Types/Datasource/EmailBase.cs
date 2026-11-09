// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type EmailBase. */
public class EmailBase : Base
{
    public long EmailTypeId { get; set; }
    public long ContactId { get; set; }
    public string Address { get; set; } = "";
    public string DisplayName { get; set; } = "";
    public bool IsPrimary { get; set; }
}
