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
    public string Name { get; set; } = "";
    public string GivenName { get; set; } = "";
    public string AdditionalName { get; set; } = "";
    public string Pronunciation { get; set; } = "";
    public string FamilyName { get; set; } = "";
    public string NickName { get; set; } = "";
    public string JobTitle { get; set; } = "";
    public string Organization { get; set; } = "";
    public bool IsStarred { get; set; }
    public bool IsFavorite { get; set; }
    public bool IsDeleted { get; set; }
    public string GenderTypeName { get; set; } = "";
    public List<Address> Addresses { get; set; } = [];
    public List<Email> Emails { get; set; } = [];
    public List<Phone> Phones { get; set; } = [];
    public Dictionary<string, string> Attributes { get; set; } = [];
    public List<Backend.Types.Datasource.Label> Labels { get; set; } = [];
}
