// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type LegacyContact. */
public class LegacyContact
{
    public string Key { get; set; } = "";
    public string FirstName { get; set; } = "";
    public string LastName { get; set; } = "";
    public string? Email { get; set; }
    public System.DateTime? ImportedAt { get; set; }
}
