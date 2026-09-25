using System.Text.Json.Serialization;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroup
{
    public int id { get; set; }
    [JsonIgnore]
    public Guid uuid { get; set; }
    [JsonIgnore]
    public DateTime created { get; set; }
    [JsonIgnore]
    public DateTime updated { get; set; }
    [JsonIgnore]
    public byte[] version { get; set; } = [];
    public string name { get; set; } = "";
    [JsonIgnore]
    public string description { get; set; } = "";
}
