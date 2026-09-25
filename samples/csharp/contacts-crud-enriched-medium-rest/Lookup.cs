namespace ContactsCrudEnrichedMediumRest;

public sealed class Lookup
{
    public int id { get; set; }
    public Guid uuid { get; set; }
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    public byte[] version { get; set; } = [];
    public string name { get; set; } = "";
    public string description { get; set; } = "";
}
