namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroupMember
{
    public int id { get; set; }
    public Guid uuid { get; set; } = Guid.NewGuid();
    public DateTime created { get; set; } = DateTime.UtcNow;
    public DateTime updated { get; set; } = DateTime.UtcNow;
    public byte[] version { get; set; } = [0];
    public int contact_id { get; set; }
    public int contact_group_id { get; set; }
}
