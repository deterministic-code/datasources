using Datasources;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class MigrateTests
{
    [Fact]
    public async Task SqliteMigrateSkipsProcedures()
    {
        var command = new MockCommand(new MockConnection { Provider = "sqlite" });
        await Migrate.Apply(command);
        Assert.Contains(command.ExecuteCalls, call => call.Text.StartsWith("PRAGMA", StringComparison.Ordinal));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("CREATE TABLE IF NOT EXISTS"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("m_Name"));
        Assert.DoesNotContain(command.ExecuteCalls, call => call.Text.Contains("CREATE PROCEDURE"));
    }

    [Fact]
    public async Task MysqlMigrateInstallsProcedures()
    {
        var command = new MockCommand(new MockConnection { Provider = "mysql" });
        await Migrate.Apply(command);
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("CREATE PROCEDURE add_contact"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("`tbl_Contacts`"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("m_FirstName"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("DROP PROCEDURE IF EXISTS delete_contact_group_member"));
        _ = new ContactsRepository(command.Connection);
        _ = new LegacyContactRepository(command.Connection);
    }

    [Fact]
    public void IfNotExistsLeavesOtherStatements() =>
        Assert.Equal("SELECT 1", Migrate.IfNotExists("SELECT 1"));
}
