using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class Migrate
{
    public static async Task Apply(ICommand command)
    {
        var provider = command.Connection.Provider;
        if (provider == "sqlite")
            await command.Execute("PRAGMA foreign_keys = ON");
        foreach (var table in Schema.Tables(provider))
            await command.Execute(IfNotExists(table.GetText()));
        await Seed(command, provider);
        if (provider == "mysql")
            foreach (var sql in MysqlProcedures.Statements())
                await command.Execute(sql);
    }

    public static string IfNotExists(string sql) =>
        sql.StartsWith("CREATE TABLE ", StringComparison.Ordinal)
            ? string.Concat("CREATE TABLE IF NOT EXISTS ", sql.AsSpan("CREATE TABLE ".Length))
            : sql;

    static async Task Seed(ICommand command, string provider)
    {
        await Lookup(command, provider, "contact_source",
        [
            (1, "Web", "Contacts entered from the web app"),
            (2, "Import", "Bulk imported contacts"),
            (3, "Legacy", "Migrated from legacy_contact")
        ]);
        await Lookup(command, provider, "contact_type",
        [
            (1, "Person", "Individual contact"),
            (2, "Organization", "Organization contact")
        ]);
        await Lookup(command, provider, "address_type",
        [
            (1, "Home", "Home address"),
            (2, "Work", "Work address"),
            (3, "Other", "Other address")
        ]);
        await Lookup(command, provider, "phone_type",
        [
            (1, "Mobile", "Mobile phone"),
            (2, "Work", "Work phone"),
            (3, "Home", "Home phone")
        ]);
        await Lookup(command, provider, "contact_groups",
        [
            (1, "Friends", "Friends"),
            (2, "Work", "Work contacts")
        ]);
    }

    static async Task Lookup(ICommand command, string provider, string table, (int Id, string Name, string Description)[] rows)
    {
        foreach (var row in rows)
        {
            var fields = new Parameter[]
            {
                new(Names.Column("id"), row.Id),
                new(Names.Column("uuid"), Guid.NewGuid().ToString()),
                new(Names.Column("created"), "2024-01-01T00:00:00Z"),
                new(Names.Column("updated"), "2024-01-01T00:00:00Z"),
                new(Names.Column("version"), new byte[] { 0 }),
                new(Names.Column("name"), row.Name),
                new(Names.Column("description"), row.Description)
            };
            await command.Execute(new InsertStatement(Schema.Ident(provider, Names.Table(table)), fields), fields);
        }
    }
}
