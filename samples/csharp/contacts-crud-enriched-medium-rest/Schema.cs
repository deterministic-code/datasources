using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class Schema
{
    static readonly NotNull Required = new();

    public static string Ident(string provider, string name) => provider == "mysql"
        ? $"`{name.Replace("`", "``")}`"
        : $"\"{name.Replace("\"", "\"\"")}\"";

    public static IReadOnlyList<CreateTableStatement> Tables(string provider)
    {
        _ = IdType(provider, "int");
        var map = ColumnTypes.Map(provider);
        var source = Ident(provider, Names.Table("contact_source"));
        var type = Ident(provider, Names.Table("contact_type"));
        var addressType = Ident(provider, Names.Table("address_type"));
        var phoneType = Ident(provider, Names.Table("phone_type"));
        var contacts = Ident(provider, Names.Table("contacts"));
        var groups = Ident(provider, Names.Table("contact_groups"));
        return
        [
            Lookup(provider, "contact_source", map),
            Lookup(provider, "contact_type", map),
            Lookup(provider, "address_type", map),
            Lookup(provider, "phone_type", map),
            new CreateTableStatement(contacts, Audit(provider, map).Concat(
            [
                Col(map, "contact_source_id", "int", required: true),
                Col(map, "contact_type_id", "int", required: true),
                Col(map, "first_name", "string", required: true),
                Col(map, "last_name", "string", required: true),
                Col(map, "email", "string"),
                Col(map, "notes", "string")
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Contacts"),
                new ForeignKey("fk_contacts_source", [Names.Column("contact_source_id")], source, [Names.Column("id")]),
                new ForeignKey("fk_contacts_type", [Names.Column("contact_type_id")], type, [Names.Column("id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("address")), Audit(provider, map).Concat(
            [
                Col(map, "contact_id", "int", required: true),
                Col(map, "address_type_id", "int", required: true),
                Col(map, "line1", "string", required: true),
                Col(map, "line2", "string"),
                Col(map, "city", "string", required: true),
                Col(map, "region", "string"),
                Col(map, "postal_code", "string"),
                Col(map, "country", "string")
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Address"),
                new ForeignKey("fk_address_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_address_type", [Names.Column("address_type_id")], addressType, [Names.Column("id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("phone")), Audit(provider, map).Concat(
            [
                Col(map, "contact_id", "int", required: true),
                Col(map, "phone_type_id", "int", required: true),
                Col(map, "number", "string", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_Phone"),
                new ForeignKey("fk_phone_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_phone_type", [Names.Column("phone_type_id")], phoneType, [Names.Column("id")])
            ]),
            Lookup(provider, "contact_groups", map),
            new CreateTableStatement(Ident(provider, Names.Table("contact_group_member")), Audit(provider, map).Concat(
            [
                Col(map, "contact_id", "int", required: true),
                Col(map, "contact_group_id", "int", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_ContactGroupMember"),
                new ForeignKey("fk_member_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")]),
                new ForeignKey("fk_member_group", [Names.Column("contact_group_id")], groups, [Names.Column("id")]),
                new UniqueConstraint("uq_contact_group_member", [Names.Column("contact_id"), Names.Column("contact_group_id")])
            ]),
            new CreateTableStatement(Ident(provider, Names.Table("legacy_contact")),
            [
                new Column(Names.Column("key"), provider == "sqlite" ? $"{map("string")} PRIMARY KEY" : map("string"), null, Required),
                Col(map, "first_name", "string", required: true),
                Col(map, "last_name", "string", required: true),
                Col(map, "email", "string"),
                Col(map, "imported_at", "datetime")
            ],
            provider == "sqlite" ? [] : [new PrimaryKey("pk_legacy_contact", [Names.Column("key")])]),
            new CreateTableStatement(Ident(provider, Names.Table("contact_change_log")), Audit(provider, map).Concat(
            [
                Col(map, "occurred_at", "datetime", required: true),
                Col(map, "contact_id", "int", required: true),
                Col(map, "changed_by", "string", required: true),
                Col(map, "change_summary", "string", required: true)
            ]).ToList(),
            [
                ..IdKey(provider, "pk_tbl_ContactChangeLog"),
                new ForeignKey("fk_change_log_contact", [Names.Column("contact_id")], contacts, [Names.Column("id")])
            ])
        ];
    }

    static CreateTableStatement Lookup(string provider, string logical, Func<string, string> map) =>
        new(Ident(provider, Names.Table(logical)), Audit(provider, map).Concat(
        [
            Col(map, "name", "string", required: true),
            Col(map, "description", "string")
        ]).ToList(), IdKey(provider, $"pk_{Names.Table(logical)}"));

    static List<Column> Audit(string provider, Func<string, string> map) =>
    [
        new Column(Names.Column("id"), IdType(provider, map("int"))),
        Col(map, "uuid", "uuid", required: true),
        Col(map, "created", "datetime", required: true),
        Col(map, "updated", "datetime", required: true),
        Col(map, "version", "binary", required: true)
    ];

    static Column Col(Func<string, string> map, string logical, string canonical, bool required = false) =>
        new(Names.Column(logical), map(canonical), null, required ? Required : null);

    static string IdType(string provider, string mapped) => provider switch
    {
        "sqlite" => $"{mapped} PRIMARY KEY",
        "mysql" => $"{mapped} AUTO_INCREMENT",
        "postgres" => $"{mapped} GENERATED BY DEFAULT AS IDENTITY",
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    static IReadOnlyList<ITableConstraint> IdKey(string provider, string name) =>
        provider == "sqlite" ? [] : [new PrimaryKey(name, [Names.Column("id")])];
}
