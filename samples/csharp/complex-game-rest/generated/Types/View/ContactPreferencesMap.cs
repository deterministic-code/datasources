// schema-version: 1.0
namespace Backend.Types.View;

/** View ContactPreferencesMap. */
public class ContactPreferencesMap
{
    public string Key { get; set; } = "";
    public LocalePreference Value { get; set; } = default!;
}
