namespace Datasources.Tests.Contact;

internal static class ContactSchema
{
    public static string Read(string dialect)
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var path = Path.Combine(dir.FullName, "csharp", "Datasources.Tests", "Contact", $"{dialect}.sql");
            if (File.Exists(path)) return File.ReadAllText(path);
        }
        throw new DirectoryNotFoundException($"contact schema {dialect}");
    }

    public static IEnumerable<string> Statements(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);
}
