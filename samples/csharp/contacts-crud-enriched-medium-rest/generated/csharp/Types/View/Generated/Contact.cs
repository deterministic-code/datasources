// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View Contact. */
public class Contact
{
    public long Id { get; set; }
    public string Uuid { get; set; } = Guid.NewGuid().ToString();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
    public string FirstName { get; set; } = "";
    public string LastName { get; set; } = "";
    public string? Email { get; set; }
    public string? Notes { get; set; }
    public string ContactSourceName { get; set; } = "";
    public string? Description { get; set; }
    public List<Address> Addresses { get; set; } = [];
    public List<Phone> Phones { get; set; } = [];
    public Dictionary<string, string> Settings { get; set; } = [];
    public Dictionary<string, LocalePreferences> Prefs { get; set; } = [];
}
