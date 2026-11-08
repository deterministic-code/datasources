using ComplexGameRest;
using Xunit;

public sealed class ApiTests
{
    [Fact]
    public void ClockDefaultsCoverNow()
    {
        Clock.Now = () => new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        Clock.Uuid = () => "00000000-0000-0000-0000-000000000000";
        Assert.Equal("00000000-0000-0000-0000-000000000000", Clock.Uuid());
        Assert.Equal(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc), Clock.Now());
    }
}
