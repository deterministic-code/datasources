namespace ContactsCrudEnrichedMediumRest;

public static class Clock
{
    public static Func<DateTime> Now { get; set; } = () => DateTime.UtcNow;
    public static Func<string> Uuid { get; set; } = () => Guid.NewGuid().ToString();
}
