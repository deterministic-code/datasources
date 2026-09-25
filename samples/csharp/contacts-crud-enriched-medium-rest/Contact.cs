using System.Text.Json.Serialization;

namespace ContactsCrudEnrichedMediumRest;

public class Contact
{
    public int id { get; set; }
    public Guid uuid { get; set; }
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    [JsonIgnore]
    public byte[] version { get; set; } = [];
    [JsonIgnore]
    public int contact_source_id { get; set; }
    [JsonIgnore]
    public int contact_type_id { get; set; }
    public string first_name { get; set; } = "";
    public string last_name { get; set; } = "";
    public string email { get; set; } = "";
    public string notes { get; set; } = "";
}
