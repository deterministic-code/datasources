using System.Text.Json.Serialization;

namespace ContactsCrudEnrichedMediumRest;

public sealed class Address
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
    public int address_type_id { get; set; }
    public string address_type { get; set; } = "";
    public string line1 { get; set; } = "";
    public string line2 { get; set; } = "";
    public string city { get; set; } = "";
    public string region { get; set; } = "";
    public string postal_code { get; set; } = "";
    public string country { get; set; } = "";
}
