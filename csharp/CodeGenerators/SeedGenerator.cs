using System.Security.Cryptography;
using System.Text;

namespace Datasources.CodeGenerators;

public static class SeedGenerator
{
    const string Namespace = "9b3a8e6c-2f1d-4a5b-8c9d-1e2f3a4b5c6d";

    public static IReadOnlyList<string> Sections(
        string dialect,
        IReadOnlyList<SqlTable> tables,
        IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> seeds,
        SqlMapping mapping)
    {
        var lines = new List<string>();
        foreach (var table in tables)
        {
            if (!seeds.TryGetValue(table.Name, out var rows) || rows.Count == 0) continue;
            var generated = SqlSchema.IsGeneratedIdentity(table) ? SqlSchema.TableIdentity(table) : [];
            var pk = table.Fields.FirstOrDefault(field => field.Name == SqlSchema.TableIdentity(table).FirstOrDefault());
            var identity = pk is not null
                && SqlSchema.UsesGeneratedIdColumn(table, pk)
                && !pk.HasDefault;
            lines.Add($"-- Seeds: {table.TableName}");
            foreach (var seed in rows)
            {
                var cols = ColsForRow(table, seed.Row, generated);
                var columns = cols.Select(c => ColumnOf(table, mapping, c, seed)).ToList();
                lines.Add(GenerateSql.Render(dialect, new SqlSeedStatement(
                    mapping.TableName(table),
                    columns,
                    "insert",
                    identity,
                    pk is null ? null : mapping.ColumnName(table, pk.Name))));
            }
        }
        return lines;
    }

    public static Dictionary<string, IReadOnlyList<SeedRow>> Added(
        IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> previous,
        IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> current)
    {
        var added = new Dictionary<string, IReadOnlyList<SeedRow>>(StringComparer.Ordinal);
        foreach (var (table, rows) in current)
        {
            var ids = previous.GetValueOrDefault(table)?.Select(row => row.Id).ToHashSet() ?? [];
            var next = rows.Where(row => !ids.Contains(row.Id)).ToList();
            if (next.Count > 0) added[table] = next;
        }
        return added;
    }

    public static IReadOnlyList<string> Deletes(
        string dialect,
        IReadOnlyList<SqlTable> tables,
        IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> seeds,
        SqlMapping mapping)
    {
        var lines = new List<string>();
        foreach (var table in tables)
        {
            if (!seeds.TryGetValue(table.Name, out var rows) || rows.Count == 0) continue;
            var id = SqlSchema.TableIdentity(table).FirstOrDefault() ?? "id";
            var field = table.Fields.FirstOrDefault(f => f.Name == id);
            lines.Add($"-- Seeds: {table.TableName}");
            foreach (var seed in rows)
                lines.Add(GenerateSql.Render(dialect, new SqlSeedStatement(
                    mapping.TableName(table),
                    [new SqlSeedColumn(mapping.ColumnName(table, id), field?.ConverterField() ?? new SpecField { Type = PkType(table) }, IdValue(table, seed.Id))],
                    "delete")));
        }
        return lines;
    }

    static string PkType(SqlTable table) =>
        table.Fields.FirstOrDefault(f => SpecTables.IsPkField(f, table.AsType(), SqlSchema.OverlayOf(table)))?.Type ?? "integer";

    static string SeedUuid(string tableName, int id)
    {
        var ns = Convert.FromHexString(Namespace.Replace("-", ""));
        var bytes = SHA1.HashData([.. ns, .. Encoding.UTF8.GetBytes($"{tableName}:{id}")])[..16];
        bytes[6] = (byte)((bytes[6] & 0x0f) | 0x50);
        bytes[8] = (byte)((bytes[8] & 0x3f) | 0x80);
        var hex = Convert.ToHexString(bytes).ToLowerInvariant();
        return $"{hex[..8]}-{hex[8..12]}-{hex[12..16]}-{hex[16..20]}-{hex[20..32]}";
    }

    static object IdValue(SqlTable table, int id)
    {
        var t = PkType(table);
        if (t == "uuid") return SeedUuid(table.Name, id);
        if (t == "string") return id.ToString();
        return id;
    }

    static SqlSeedColumn ColumnOf(SqlTable table, SqlMapping mapping, string logical, SeedRow seed)
    {
        var field = table.Fields.FirstOrDefault(f => f.Name == logical);
        var spec = field?.ConverterField() ?? new SpecField { Type = "string", Name = logical };
        var generated = SqlSchema.IsGeneratedIdentity(table) ? SqlSchema.TableIdentity(table) : [];
        object? value = generated.Contains(logical) ? IdValue(table, seed.Id)
            : logical == "uuid" ? SeedUuid(table.Name, seed.Id)
            : seed.Row.GetValueOrDefault(logical);
        return new SqlSeedColumn(mapping.ColumnName(table, logical), spec, value);
    }

    static List<string> ColsForRow(SqlTable table, IReadOnlyDictionary<string, object?> row, IReadOnlyList<string> generated)
    {
        var names = table.Fields.Select(f => f.Name).ToHashSet(StringComparer.Ordinal);
        return
        [
            .. generated.Where(names.Contains),
            .. names.Contains("uuid") && !generated.Contains("uuid") ? ["uuid"] : Array.Empty<string>(),
            .. row.Keys.Where(k => !generated.Contains(k) && k != "uuid"),
        ];
    }
}
