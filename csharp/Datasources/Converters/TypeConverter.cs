using System.Collections;
using System.Globalization;
using System.Reflection;

namespace Datasources;

public static class TypeConverter
{
    const string Host = "csharp";
    static readonly Dictionary<string, ProviderTypes> Registry = [];

    public static void Use(string source, ProviderTypes types) => Registry[source] = types;
    public static void Reset() => Registry.Clear();

    public static TypeMapping<T> Mapping<T>(
        string source,
        string destination = Host,
        IReadOnlyDictionary<string, string>? schema = null,
        IReadOnlyDictionary<string, IFieldConverter>? converters = null,
        IReadOnlyDictionary<string, string>? names = null,
        string? name = null) where T : new()
    {
        if (destination != Host)
            throw new DatasourceException($"TypeConverter destination must be \"{Host}\"");
        if (!Registry.TryGetValue(source, out var table))
            throw new DatasourceException($"Provider \"{source}\" is not registered");
        var inferred = typeof(IDictionary).IsAssignableFrom(typeof(T))
            ? (Schema: new Dictionary<string, string>(), Names: new Dictionary<string, string>())
            : InferSchema(typeof(T), table.Rename);
        var merged = new Dictionary<string, string>(inferred.Schema);
        if (schema is not null)
            foreach (var (key, value) in schema)
                merged[key] = value;
        var namePairs = new Dictionary<string, string>(inferred.Names);
        if (names is not null)
            foreach (var (key, value) in names)
                namePairs[key] = value;
        foreach (var key in merged.Keys)
            if (!namePairs.ContainsKey(key))
                namePairs[key] = table.Rename ? Camel(key) : key;
        var mapped = DatasourceMapper.Mapping(source, Host, namePairs);
        IFieldConverter ConverterFor(string sqlName)
        {
            if (converters is not null && converters.TryGetValue(sqlName, out var overrideConverter))
                return overrideConverter;
            return merged.TryGetValue(sqlName, out var sqlType)
                && table.Types.TryGetValue(Normalize(sqlType), out var typed)
                    ? typed
                    : FieldConverters.Identity;
        }
        return new TypeMapping<T>
        {
            Name = name ?? typeof(T).Name,
            Source = source,
            Destination = Host,
            Schema = merged,
            To = row =>
            {
                var target = new T();
                if (target is IDictionary<string, Value> dict)
                {
                    foreach (var (sqlName, value) in row)
                        dict[DestName(mapped, sqlName, table.Rename)] = ConverterFor(sqlName).To(value);
                    return target;
                }
                foreach (var (sqlName, value) in row)
                {
                    var prop = typeof(T).GetProperty(DestName(mapped, sqlName, table.Rename));
                    if (prop is null || !prop.CanWrite) continue;
                    prop.SetValue(target, Assign(ConverterFor(sqlName).To(value), prop.PropertyType));
                }
                return target;
            },
            From = obj =>
            {
                var result = new Dictionary<string, Value>();
                if (obj is IDictionary<string, Value> dict)
                {
                    foreach (var (destName, value) in dict)
                    {
                        var sqlName = SqlName(mapped, destName, table.Rename);
                        result[sqlName] = ConverterFor(sqlName).From(value);
                    }
                    return result;
                }
                foreach (var prop in DataProperties(typeof(T)))
                {
                    var sqlName = SqlName(mapped, prop.Name, table.Rename);
                    result[sqlName] = ConverterFor(sqlName).From(Value.From(prop.GetValue(obj)));
                }
                return result;
            }
        };
    }

    public static Dictionary<string, IFieldConverter> CanonicalConverters(string booleanStorage = "integer") => new()
    {
        ["TEXT"] = FieldConverters.Identity,
        ["INTEGER"] = FieldConverters.Identity,
        ["REAL"] = FieldConverters.Identity,
        ["NUMERIC"] = FieldConverters.Decimal(),
        ["BOOLEAN"] = FieldConverters.Boolean(booleanStorage),
        ["DATETIME"] = FieldConverters.DateTime,
        ["BLOB"] = FieldConverters.Binary,
        ["UUID"] = FieldConverters.Uuid,
    };

    public static ProviderTypes SqlProviderTypes(
        IReadOnlyDictionary<string, string>? aliases = null,
        bool rename = true,
        string booleanStorage = "integer")
    {
        var types = CanonicalConverters(booleanStorage);
        if (aliases is not null)
            foreach (var (alias, target) in aliases)
                types[alias] = types.TryGetValue(target, out var converter) ? converter : FieldConverters.Identity;
        return new ProviderTypes { Rename = rename, Types = types };
    }

    public static ProviderTypes IdentityProviderTypes() => new() { Rename = false, Types = [] };

    public static string InferSqlType(Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        if (type == typeof(string)) return "TEXT";
        if (type == typeof(bool)) return "BOOLEAN";
        if (type == typeof(DateTime)) return "DATETIME";
        if (type == typeof(Guid)) return "UUID";
        if (type == typeof(byte[])) return "BLOB";
        if (type == typeof(float) || type == typeof(double)) return "REAL";
        if (type == typeof(decimal)) return "NUMERIC";
        if (type == typeof(byte) || type == typeof(sbyte) || type == typeof(short) || type == typeof(ushort)
            || type == typeof(int) || type == typeof(uint) || type == typeof(long) || type == typeof(ulong))
            return "INTEGER";
        return "TEXT";
    }

    public static string Normalize(string sqlType)
    {
        var trimmed = sqlType.Trim().ToUpperInvariant();
        if (trimmed.StartsWith("TINYINT(1)")) return "TINYINT(1)";
        if (trimmed.StartsWith("CHAR(36)")) return "CHAR(36)";
        var paren = trimmed.IndexOf('(');
        return paren == -1 ? trimmed : trimmed[..paren].Trim();
    }

    static (Dictionary<string, string> Schema, Dictionary<string, string> Names) InferSchema(Type type, bool rename)
    {
        var schema = new Dictionary<string, string>();
        var names = new Dictionary<string, string>();
        foreach (var prop in DataProperties(type))
        {
            var sqlName = rename ? Snake(prop.Name) : prop.Name;
            schema[sqlName] = InferSqlType(prop.PropertyType);
            names[sqlName] = prop.Name;
        }
        return (schema, names);
    }

    static IEnumerable<PropertyInfo> DataProperties(Type type) =>
        type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0);

    static string Snake(string name)
    {
        var chars = new List<char>();
        for (var i = 0; i < name.Length; i++)
        {
            var ch = name[i];
            if (i > 0 && char.IsUpper(ch) && (char.IsLower(name[i - 1]) || char.IsDigit(name[i - 1])
                || (i + 1 < name.Length && char.IsLower(name[i + 1]))))
                chars.Add('_');
            chars.Add(char.ToLowerInvariant(ch));
        }
        return new string([.. chars]);
    }

    static string DestName(DatasourceMapper mapped, string sqlName, bool rename)
    {
        var dest = mapped.To(sqlName);
        return dest != sqlName || !rename ? dest : Pascal(sqlName);
    }

    static string SqlName(DatasourceMapper mapped, string destName, bool rename)
    {
        var sql = mapped.From(destName);
        return sql != destName || !rename ? sql : Snake(destName);
    }

    static string Camel(string name)
    {
        var parts = name.Split('_', StringSplitOptions.RemoveEmptyEntries);
        if (parts.Length == 0) return name;
        return parts[0] + string.Concat(parts.Skip(1).Select(part =>
            char.ToUpperInvariant(part[0]) + part[1..]));
    }

    static object? Assign(Value value, Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        var raw = value.ToObject();
        if (raw is null) return null;
        if (type.IsInstanceOfType(raw)) return raw;
        if (type == typeof(DateTime) && raw is string text)
            return DateTime.Parse(text, CultureInfo.InvariantCulture, DateTimeStyles.RoundtripKind);
        if (type == typeof(Guid) && raw is string guid) return Guid.Parse(guid);
        if (type == typeof(byte[]) && raw is string bytes) return Convert.FromBase64String(bytes);
        if (type == typeof(decimal) && raw is string number)
            return decimal.Parse(number, CultureInfo.InvariantCulture);
        if (type == typeof(bool) && raw is long flag) return flag != 0;
        if (raw is long integer)
        {
            if (type == typeof(int)) return (int)integer;
            if (type == typeof(byte)) return (byte)integer;
            if (type == typeof(short)) return (short)integer;
            if (type == typeof(float)) return (float)integer;
            if (type == typeof(double)) return (double)integer;
        }
        return Convert.ChangeType(raw, type, CultureInfo.InvariantCulture);
    }

    static string Pascal(string name)
    {
        var camel = Camel(name);
        return camel.Length == 0 ? camel : char.ToUpperInvariant(camel[0]) + camel[1..];
    }
}
