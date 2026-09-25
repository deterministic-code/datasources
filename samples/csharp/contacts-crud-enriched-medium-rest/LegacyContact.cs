namespace ContactsCrudEnrichedMediumRest;

public sealed class LegacyContact
{
    public string key { get; set; } = "";
    public string first_name { get; set; } = "";
    public string last_name { get; set; } = "";
    public string email { get; set; } = "";
    public DateTime? imported_at { get; set; }
}
