using System.Text.Json.Serialization;

namespace ContactsCrudEnrichedMediumRest;

public sealed class Phone
{
    public int id { get; set; }
    public Guid uuid { get; set; }
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    [JsonIgnore]
    public byte[] version { get; set; } = [];
    [JsonIgnore]
    public int contact_id { get; set; }
    [JsonIgnore]
    public int phone_type_id { get; set; }
    public string phone_type { get; set; } = "";
    public string number { get; set; } = "";
}
