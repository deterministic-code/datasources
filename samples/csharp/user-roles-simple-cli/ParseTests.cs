namespace UserRolesSimpleCli.Tests;

public sealed class ParseTests
{
    [Fact]
    public void ParsesCommands()
    {
        Assert.Equal(("add-user", new[] { "a", "b" }), Program.Parsed(["add-user", "a", "b"]));
        Assert.Equal(("add-role", new[] { "admin" }), Program.Parsed(["add-role", "admin"]));
        Assert.Equal(("add-group", new[] { "staff" }), Program.Parsed(["add-group", "staff"]));
        Assert.Equal(("add-user-role", new[] { "a", "admin" }), Program.Parsed(["add-user-role", "a", "admin"]));
        Assert.Equal(("add-user-group", new[] { "a", "staff" }), Program.Parsed(["add-user-group", "a", "staff"]));
        Assert.Equal(("enable-user", new[] { "a" }), Program.Parsed(["enable-user", "a"]));
        Assert.Equal(("disable-user", new[] { "a" }), Program.Parsed(["disable-user", "a"]));
        Assert.Equal(("list-users-roles-groups", Array.Empty<string>()), Program.Parsed(["list-users-roles-groups"]));
    }

    [Theory]
    [InlineData("add-user")]
    [InlineData("add-role")]
    [InlineData("add-group")]
    [InlineData("add-user-role")]
    [InlineData("add-user-group")]
    [InlineData("enable-user")]
    [InlineData("disable-user")]
    [InlineData("nope")]
    public void RejectsBadArgs(string verb) =>
        Assert.Throws<InvalidOperationException>(() => Program.Parsed([verb]));

    [Fact]
    public void RejectsEmpty() =>
        Assert.Contains("list-users-roles-groups", Assert.Throws<InvalidOperationException>(() => Program.Parsed([])).Message);

    [Fact]
    public void RejectsExtraArgs()
    {
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["add-user", "a", "b", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["add-role", "admin", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["add-group", "staff", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["add-user-role", "a", "admin", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["add-user-group", "a", "staff", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["enable-user", "a", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["disable-user", "a", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Program.Parsed(["list-users-roles-groups", "extra"]));
    }
}
