namespace ComplexGameRest;

public static class Clock
{
    public static Func<DateTime> Now { get; set; } = () => DateTime.UtcNow;
    public static Func<Guid> Uuid { get; set; } = () => Guid.NewGuid();
}
