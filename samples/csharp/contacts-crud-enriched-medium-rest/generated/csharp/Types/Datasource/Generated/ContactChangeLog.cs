// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactChangeLog. */
public class ContactChangeLog
{
    public long Id { get; set; }
    public System.DateTime OccurredAt { get; set; }
    public long ContactId { get; set; }
    public string ChangedBy { get; set; } = "";
    public string ChangeSummary { get; set; } = "";
}
