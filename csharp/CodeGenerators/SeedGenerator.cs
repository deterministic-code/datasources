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
            var pk = PkType(table);
            var identity = SqlSchema.IsGeneratedIdentity(table) && pk is not "uuid" and not "string";
            var inserts = rows.Select(seed =>
            {
                var cols = ColsForRow(table, seed.Row, generated);
                var values = cols.Select(c =>
                    generated.Contains(c) ? IdValue(table, seed.Id)
                    : c == "uuid" ? DefaultValueConverter.SqlStringLiteral(SeedUuid(table.Name, seed.Id))
                    : ColValue(dialect, table.Fields.FirstOrDefault(f => f.Name == c), seed.Row.GetValueOrDefault(c))).ToList();
                return TableStatements.SeedInsert(table, mapping, cols, values, dialect);
            }).ToList();
            lines.Add($"-- Seeds: {table.TableName}");
            lines.Add(SqlDialect.RenderSql(dialect, TableStatements.IdentityInsert(dialect, table, mapping, inserts, identity)));
            lines.Add("");
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

    static string RenderValue(string dialect, SpecTypeField field, object? value)
    {
        if (field.Type == "boolean")
            return SqlDialect.DriverOf(dialect).DefaultValueConverter.Convert("boolean", Truthy(value) ? "true" : "false") ?? "0";
        if (field.Type is "number" or "reference") return Convert.ToString(value) ?? "NULL";
        return DefaultValueConverter.SqlStringLiteral(value);
    }

    static bool Truthy(object? value) => value is true or "true" or "True" or 1 or 1L;

    static string IdValue(SqlTable table, int id)
    {
        var t = PkType(table);
        if (t == "uuid") return DefaultValueConverter.SqlStringLiteral(SeedUuid(table.Name, id));
        if (t == "string") return DefaultValueConverter.SqlStringLiteral(id);
        return id.ToString();
    }

    static string ColValue(string dialect, SpecTypeField? field, object? value)
    {
        if (field is null) return "NULL";
        if (value is null) return SqlDialect.SqlDefault(dialect, field.ConverterField()) ?? "NULL";
        return RenderValue(dialect, field, value);
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
