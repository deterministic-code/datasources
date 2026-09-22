namespace UserRolesSimpleCli.Tests;

public sealed class ParseTests
{
    [Fact]
    public void ParsesCommands()
    {
        Assert.Equal(new CliCommand.AddUser("a", "b"), Parse.Args(["add-user", "a", "b"]));
        Assert.Equal(new CliCommand.AddRole("admin"), Parse.Args(["add-role", "admin"]));
        Assert.Equal(new CliCommand.AddGroup("staff"), Parse.Args(["add-group", "staff"]));
        Assert.Equal(new CliCommand.AddUserRole("a", "admin"), Parse.Args(["add-user-role", "a", "admin"]));
        Assert.Equal(new CliCommand.AddUserGroup("a", "staff"), Parse.Args(["add-user-group", "a", "staff"]));
        Assert.Equal(new CliCommand.EnableUser("a"), Parse.Args(["enable-user", "a"]));
        Assert.Equal(new CliCommand.DisableUser("a"), Parse.Args(["disable-user", "a"]));
        Assert.Equal(new CliCommand.ListUsersRolesGroups(), Parse.Args(["list-users-roles-groups"]));
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
        Assert.Throws<InvalidOperationException>(() => Parse.Args([verb]));

    [Fact]
    public void RejectsEmpty() =>
        Assert.Contains("list-users-roles-groups", Assert.Throws<InvalidOperationException>(() => Parse.Args([])).Message);

    [Fact]
    public void RejectsExtraArgs()
    {
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["add-user", "a", "b", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["add-role", "admin", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["add-group", "staff", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["add-user-role", "a", "admin", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["add-user-group", "a", "staff", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["enable-user", "a", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["disable-user", "a", "extra"]));
        Assert.Throws<InvalidOperationException>(() => Parse.Args(["list-users-roles-groups", "extra"]));
    }
}
