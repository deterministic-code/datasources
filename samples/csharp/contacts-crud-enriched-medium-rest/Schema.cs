using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class Schema
{
    static readonly NotNull Required = new();

    public static string Ident(string provider, string name) => provider == "mysql"
        ? $"`{name.Replace("`", "``")}`"
        : $"\"{name.Replace("\"", "\"\"")}\"";

    public static IReadOnlyList<CreateTableStatement> Tables(string provider, IDriver driver)
    {
        var source = Ident(provider, Names.Table("contact_source"));
        var type = Ident(provider, Names.Table("contact_type"));
        var addressType = Ident(provider, Names.Table("address_type"));
        var phoneType = Ident(provider, Names.Table("phone_type"));
        var contacts = Ident(provider, Names.Table("contacts"));
        var groups = Ident(provider, Names.Table("contact_groups"));
        return
        [
            Lookup(provider, "contact_source", driver),
            Lookup(provider, "contact_type", driver),
            Lookup(provider, "address_type", driver),
            Lookup(provider, "phone_type", driver),
            new CreateTableStatement(contacts, Audit(provider, driver).Concat(
            [
                Col(provider, driver, "contact_source_id", "int", required: true),
                Col(provider, driver, "contact_type_id", "int", required: true),
                Col(provider, driver, "first_name", "string", required: true),
                Col(provider, driver, "last_name", "string", required: true),
                Col(provider, driver, "email", "string"),
                Col(provider, driver, "notes", "string")
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Contacts"),
                new ForeignKey("fk_contacts_source", [Names.Column("contact_source_id")], source, [Names.Column("id")]),
                new ForeignKey("fk_contacts_type", [Names.Column("contact_type_id")], type, [Names.Column("id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("address")), Audit(provider, driver).Concat(
            [
                Col(provider, driver, "contact_id", "int", required: true),
                Col(provider, driver, "address_type_id", "int", required: true),
                Col(provider, driver, "line1", "string", required: true),
                Col(provider, driver, "line2", "string"),
                Col(provider, driver, "city", "string", required: true),
                Col(provider, driver, "region", "string"),
                Col(provider, driver, "postal_code", "string"),
                Col(provider, driver, "country", "string")
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Address"),
                new ForeignKey("fk_address_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_address_type", [Names.Column("address_type_id")], addressType, [Names.Column("id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("phone")), Audit(provider, driver).Concat(
            [
                Col(provider, driver, "contact_id", "int", required: true),
                Col(provider, driver, "phone_type_id", "int", required: true),
                Col(provider, driver, "number", "string", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Phone"),
                new ForeignKey("fk_phone_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_phone_type", [Names.Column("phone_type_id")], phoneType, [Names.Column("id")])
            ]),
            Lookup(provider, "contact_groups", driver),
            new CreateTableStatement(Ident(provider, Names.Table("contact_group_member")), Audit(provider, driver).Concat(
            [
                Col(provider, driver, "contact_id", "int", required: true),
                Col(provider, driver, "contact_group_id", "int", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_ContactGroupMember"),
                new ForeignKey("fk_member_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_member_group", [Names.Column("contact_group_id")], groups, [Names.Column("id")]),
                new UniqueConstraint("uq_contact_group_member", [Names.Column("contact_id"), Names.Column("contact_group_id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("legacy_contact")),
            [
                new Column(Names.Column("key"), provider == "sqlite" ? $"{driver.ColumnType("string")} PRIMARY KEY" : "string", null, Required),
                Col(provider, driver, "first_name", "string", required: true),
                Col(provider, driver, "last_name", "string", required: true),
                Col(provider, driver, "email", "string"),
                Col(provider, driver, "imported_at", "datetime")
            ],
            provider == "sqlite" ? [] : [new PrimaryKey("pk_legacy_contact", [Names.Column("key")])]),
            new CreateTableStatement(Ident(provider, Names.Table("contact_change_log")), Audit(provider, driver).Concat(
            [
                Col(provider, driver, "occurred_at", "datetime", required: true),
                Col(provider, driver, "contact_id", "int", required: true),
                Col(provider, driver, "changed_by", "string", required: true),
                Col(provider, driver, "change_summary", "string", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_ContactChangeLog"),
                new ForeignKey("fk_change_log_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")])
            ])
        ];
    }

    static CreateTableStatement Lookup(string provider, string logical, IDriver driver) =>
        new(Ident(provider, Names.Table(logical)), Audit(provider, driver).Concat(
        [
            Col(provider, driver, "name", "string", required: true),
            Col(provider, driver, "description", "string")
        ]).ToList(), IdKey(provider, $"pk_{Names.Table(logical)}"));

    static List<Column> Audit(string provider, IDriver driver) =>
    [
        IdColumn(provider, driver),
        Col(provider, driver, "uuid", "uuid", required: true),
        Col(provider, driver, "created", "datetime", required: true),
        Col(provider, driver, "updated", "datetime", required: true),
        Col(provider, driver, "version", "binary", required: true)
    ];

    static Column Col(string provider, IDriver driver, string logical, string canonical, bool required = false) =>
        new(Names.Column(logical), provider == "sqlite" ? driver.ColumnType(canonical) : canonical, null, required ? Required : null);

    static Column IdColumn(string provider, IDriver driver) => provider switch
    {
        "sqlite" => new Column(Names.Column("id"), $"{driver.ColumnType("int")} PRIMARY KEY"),
        "mysql" or "postgres" => new Column(Names.Column("id"), "int", Identity: true),
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    static IReadOnlyList<ITableConstraint> IdKey(string provider, string name) =>
        provider == "sqlite" ? [] : [new PrimaryKey(name, [Names.Column("id")])];
}
