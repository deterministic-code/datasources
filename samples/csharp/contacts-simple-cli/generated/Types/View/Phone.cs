// schema-version: 1.0
using System.Collections.Generic;

namespace Backend.Types.View;

/** View Phone. */
public class Phone
{
    public long Id { get; set; }
    public System.Guid Uuid { get; set; } = Guid.NewGuid();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
    public long ContactId { get; set; }
    public string Number { get; set; } = "";
    public string DisplayName { get; set; } = "";
    public string Extention { get; set; } = "";
    public bool IsPrimary { get; set; }
    public bool IsVerified { get; set; }
    public string PhoneTypeName { get; set; } = "";
    public string Description { get; set; } = "";
    public Dictionary<string, string> Attributes { get; set; } = [];
    public List<Backend.Types.Datasource.Label> Labels { get; set; } = [];
}
