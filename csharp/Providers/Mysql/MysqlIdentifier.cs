using System.Text;

namespace Datasources.Mysql;

internal static class MysqlIdentifier
{
    const int Limit = 64;

    static readonly string[] Roles =
    [
        "default_constraint",
        "unique_constraint",
        "foreign_key",
        "primary_key",
        "updated_at",
        "index",
    ];

    /// <summary>
    /// Leave a name that fits. A longer one keeps its role suffix and a hash of the
    /// full name, so create and drop statements spell it the same way.
    /// </summary>
    internal static string Fit(string name)
    {
        if (name.Length <= Limit) return name;
        var suffix = Suffix(name);
        var tail = (suffix.StartsWith('-') ? "-" : "_") + Hash(name) + suffix;
        return name[..(Limit - tail.Length)].TrimEnd('_', '-') + tail;
    }

    internal static ITableConstraint Fit(ITableConstraint constraint) => constraint switch
    {
        PrimaryKey key => key with { Name = Fit(key.Name) },
        ForeignKey key => key with { Name = Fit(key.Name) },
        UniqueConstraint key => key with { Name = Fit(key.Name) },
        CheckConstraint key => key with { Name = Fit(key.Name) },
        _ => throw new InvalidOperationException("empty constraint"),
    };

    static string Suffix(string name)
    {
        var lower = name.ToLowerInvariant();
        foreach (var role in Roles)
        {
            var snake = "_" + role;
            var kebab = "-" + role.Replace('_', '-');
            if (lower.EndsWith(snake, StringComparison.Ordinal)) return name[^snake.Length..];
            if (lower.EndsWith(kebab, StringComparison.Ordinal)) return name[^kebab.Length..];
        }
        foreach (var role in Roles)
        {
            var shaped = Pascal(role);
            if (name.EndsWith(shaped, StringComparison.Ordinal)) return name[^shaped.Length..];
        }
        return "";
    }

    static string Pascal(string role) =>
        string.Concat(role.Split('_').Select(word => char.ToUpperInvariant(word[0]) + word[1..]));

    /// <summary>First 6 hex digits of FNV-1a 32-bit. The same function in TypeScript and Rust.</summary>
    static string Hash(string name)
    {
        var hash = 2166136261u;
        foreach (var b in Encoding.UTF8.GetBytes(name))
        {
            hash ^= b;
            hash *= 16777619u;
        }
        return hash.ToString("x8")[..6];
    }
}
