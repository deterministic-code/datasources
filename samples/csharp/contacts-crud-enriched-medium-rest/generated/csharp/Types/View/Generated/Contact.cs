// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View Contact. */
public class Contact : Backend.Types.Datasource.ContactsBase
{
    public string ContactSourceName { get; set; } = "";
    public string? Description { get; set; }
    public List<Address> Addresses { get; set; } = [];
    public List<Phone> Phones { get; set; } = [];
    public Dictionary<string, string> Settings { get; set; } = [];
    public Dictionary<string, LocalePreferences> Prefs { get; set; } = [];
}
