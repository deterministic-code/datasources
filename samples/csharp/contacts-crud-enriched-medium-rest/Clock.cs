namespace ContactsCrudEnrichedMediumRest;

public static class Clock
{
    public static Func<DateTime> UtcNow { get; set; } = () => DateTime.UtcNow;

    public static Func<Guid> Uuid { get; set; } = Guid.NewGuid;
}
