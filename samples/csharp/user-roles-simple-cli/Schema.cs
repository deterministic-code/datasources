using Datasources;

namespace UserRolesSimpleCli;

public static class Schema
{
    public static string Ident(string provider, string name) => Quote(provider, name);

    public static string Qualified(string provider, string name) =>
        string.Join(".", name.Split('.').Select(part => Quote(provider, part)));

    static string Quote(string provider, string id) => provider == "mysql"
        ? $"`{id.Replace("`", "``")}`"
        : $"\"{id.Replace("\"", "\"\"")}\"";

    public static Value Flag(string provider, bool value) => provider == "sqlite" ? value ? 1 : 0 : value;

    public static IReadOnlyList<CreateTableStatement> Tables(string provider, IDriver driver)
    {
        var q = (string name) => Ident(provider, name);
        var id = IdColumn(provider, driver);
        var text = Stored(provider, driver, "string");
        var flag = Stored(provider, driver, "bool");
        var created = Stored(provider, driver, "datetime");
        var updated = Stored(provider, driver, "binary");
        var number = Stored(provider, driver, "int");
        var enabled = new ColumnDefault("1");
        var required = new NotNull();
        return
        [
            new CreateTableStatement(q("users"),
            [
                id,
                new Column("username", text, null, required),
                new Column("password", text, null, required),
                new Column("is_enabled", flag, enabled, required),
                new Column("created", created, null, required),
                new Column("updated", updated, null, required)
            ], IdKey(provider, "pk_users")),
            new CreateTableStatement(q("roles"),
            [
                id,
                new Column("name", text, null, required),
                new Column("is_enabled", flag, enabled, required),
                new Column("created", created, null, required),
                new Column("updated", updated, null, required)
            ], IdKey(provider, "pk_roles")),
            new CreateTableStatement(q("groups"),
            [
                id,
                new Column("ugly_name", text, null, required),
                new Column("is_enabled", flag, enabled, required),
                new Column("created", created, null, required),
                new Column("updated", updated, null, required)
            ], IdKey(provider, "pk_groups")),
            new CreateTableStatement(q("user_groups"),
            [
                id,
                new Column("user_id", number, null, required),
                new Column("group_id", number, null, required),
                new Column("created", created, null, required),
                new Column("updated", updated, null, required)
            ],
            [
                ..IdKey(provider, "pk_user_groups"),
                new ForeignKey("fk_user_groups_user", ["user_id"], q("users"), ["id"]),
                new ForeignKey("fk_user_groups_group", ["group_id"], q("groups"), ["id"])
            ]),
            new CreateTableStatement(q("user_roles"),
            [
                id,
                new Column("user_id", number, null, required),
                new Column("bad_role_id", number, null, required),
                new Column("created", created, null, required),
                new Column("updated", updated, null, required)
            ],
            [
                ..IdKey(provider, "pk_user_roles"),
                new ForeignKey("fk_user_roles_user", ["user_id"], q("users"), ["id"]),
                new ForeignKey("fk_user_roles_role", ["bad_role_id"], q("roles"), ["id"])
            ])
        ];
    }

    public static IReadOnlyList<string> MysqlProcedures()
    {
        using var stream = typeof(Schema).Assembly.GetManifestResourceStream("mysql-procedures.sql")!;
        using var reader = new StreamReader(stream);
        return reader.ReadToEnd().Split("---", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
    }

    static string Stored(string provider, IDriver driver, string canonical) =>
        provider == "sqlite" ? driver.ColumnType(canonical) : canonical;

    static Column IdColumn(string provider, IDriver driver) => provider switch
    {
        "sqlite" => new Column("id", $"{driver.ColumnType("int")} PRIMARY KEY"),
        "mysql" or "postgres" => new Column("id", "int", Identity: true),
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    static IReadOnlyList<ITableConstraint> IdKey(string provider, string name) =>
        provider == "sqlite" ? [] : [new PrimaryKey(name, ["id"])];
}
