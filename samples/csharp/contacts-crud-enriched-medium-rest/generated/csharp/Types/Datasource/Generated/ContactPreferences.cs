// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ContactPreferences. */
public class ContactPreferences : Base
{
    public long ContactId { get; set; }
    public string Key { get; set; } = "";
    public string Locale { get; set; } = "";
    public string Timezone { get; set; } = "";
}
