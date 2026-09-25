namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroupMember
{
    public int id { get; set; }
    public Guid uuid { get; set; } = Clock.Uuid();
    public DateTime created { get; set; } = Clock.UtcNow();
    public DateTime updated { get; set; } = Clock.UtcNow();
    public byte[] version { get; set; } = [0];
    public int contact_id { get; set; }
    public int contact_group_id { get; set; }
}
