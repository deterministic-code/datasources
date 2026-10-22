// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactsBase. */
public class ContactsBase : Base
{
    public long ContactSourceId { get; set; }
    public string FirstName { get; set; } = "";
    public string LastName { get; set; } = "";
    public string? Email { get; set; }
    public string? Notes { get; set; }
}
