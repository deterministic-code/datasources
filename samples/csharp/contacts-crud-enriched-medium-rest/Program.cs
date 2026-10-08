using Datasources;
using Datasources.Sqlite;

var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?>
{
    ["DATASOURCE_URL"] = "sqlite:///:memory:",
});
RepositoryRegistry.Register(ds.Repositories);
var command = await ds.CreateCommand();
await command.Execute(new Native("PRAGMA foreign_keys = ON"));
foreach (var sql in Statements.Of(await File.ReadAllTextAsync(args[0])))
    await command.Execute(new Native(sql));
dynamic d = ds;
var contacts = (ICRUDRepository<Backend.Types.View.Contact>)d.Contacts.Create();
var created = await contacts.Create(new Backend.Types.View.Contact
{
    FirstName = "Ada",
    LastName = "Lovelace",
    ContactSourceId = 1,
    Uuid = "00000000-0000-0000-0000-000000000001",
    Version = [0],
    Addresses = [],
    Phones = [],
    Settings = [],
    Prefs = [],
});
if (created.FirstName != "Ada")
    throw new InvalidOperationException("create");
var loaded = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", created.Id)));
if (loaded?.FirstName != "Ada")
    throw new InvalidOperationException("read");

static class Statements
{
    public static IEnumerable<string> Of(string sql)
    {
        var cleaned = string.Join('\n', sql.Split('\n').Where(line => !line.TrimStart().StartsWith("--", StringComparison.Ordinal)));
        var outList = new List<string>();
        var depth = 0;
        var start = 0;
        for (var i = 0; i < cleaned.Length; i++)
        {
            if (Word(cleaned, i, "BEGIN")) depth++;
            if (cleaned[i] != ';') continue;
            if (EndsWithWord(cleaned, start, i, "END") && depth > 0) depth--;
            if (depth != 0) continue;
            var text = WithSeedVersion(cleaned[start..(i + 1)].Trim());
            if (text.Length > 0) outList.Add(text);
            start = i + 1;
        }
        return outList;
    }

    static string WithSeedVersion(string sql)
    {
        if (!sql.StartsWith("INSERT INTO", StringComparison.Ordinal) || sql.Contains("\"version\"", StringComparison.Ordinal))
            return sql;
        var values = sql.IndexOf(") VALUES (", StringComparison.Ordinal);
        return values < 0
            ? sql
            : $"{sql[..values]}, \"version\") VALUES ({sql[(values + ") VALUES (".Length)..^2]}, X'00');";
    }

    static bool Word(string sql, int i, string token)
    {
        if (!sql.AsSpan(i).StartsWith(token, StringComparison.Ordinal)) return false;
        if (i > 0 && char.IsLetterOrDigit(sql[i - 1])) return false;
        var end = i + token.Length;
        return end == sql.Length || !char.IsLetterOrDigit(sql[end]);
    }

    static bool EndsWithWord(string sql, int start, int semicolon, string token)
    {
        var i = semicolon - 1;
        while (i >= start && char.IsWhiteSpace(sql[i])) i--;
        if (i < start + token.Length - 1 || sql.AsSpan(i - token.Length + 1, token.Length).ToString() != token)
            return false;
        var before = i - token.Length;
        return before < start || !char.IsLetterOrDigit(sql[before]);
    }
}
