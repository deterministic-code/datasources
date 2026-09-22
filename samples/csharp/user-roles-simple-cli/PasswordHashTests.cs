namespace UserRolesSimpleCli.Tests;

public sealed class PasswordHashTests
{
    [Fact]
    public void HashesPassword()
    {
        var hash = PasswordHash.Hash("secret");
        Assert.NotEqual("secret", hash);
        Assert.True(BCrypt.Net.BCrypt.Verify("secret", hash));
    }
}
