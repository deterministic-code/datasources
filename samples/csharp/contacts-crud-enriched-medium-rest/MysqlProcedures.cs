namespace ContactsCrudEnrichedMediumRest;

public static class MysqlProcedures
{
    public static IReadOnlyList<string> Statements()
    {
        var sql = new List<string>();
        foreach (var (name, table, columns, where) in Specs())
        {
            sql.Add($"DROP PROCEDURE IF EXISTS {name}");
            sql.Add(Create(name, table, columns, where));
        }
        return sql;
    }

    static string Create(string name, string table, string[] columns, string[] where)
    {
        var parameters = columns.Concat(where).Select(column => $"IN p_{Names.Column(column)} {MysqlType(column)}");
        var fields = string.Join(", ", columns.Select(column => $"`{Names.Column(column)}`"));
        var values = string.Join(", ", columns.Select(column => $"p_{Names.Column(column)}"));
        var filter = string.Join(" AND ", where.Select(column => $"`{Names.Column(column)}` = p_{Names.Column(column)}"));
        var body = name.StartsWith("add_", StringComparison.Ordinal)
            ? $"INSERT INTO `{table}` ({fields}) VALUES ({values})"
            : name.StartsWith("update_", StringComparison.Ordinal)
                ? $"UPDATE `{table}` SET {string.Join(", ", columns.Select(column => $"`{Names.Column(column)}` = p_{Names.Column(column)}"))} WHERE {filter}"
                : $"DELETE FROM `{table}` WHERE {filter}";
        return $"CREATE PROCEDURE {name}({string.Join(", ", parameters)})\nBEGIN\n  {body};\nEND";
    }

    static string MysqlType(string column) => column == "version" ? "BLOB" : column.EndsWith("_id", StringComparison.Ordinal) || column == "id" ? "INT" : "TEXT";

    static IEnumerable<(string Name, string Table, string[] Columns, string[] Where)> Specs() =>
    [
        ("add_contact", "tbl_Contacts", ContactColumns, []),
        ("update_contact", "tbl_Contacts", ContactColumns, ["id"]),
        ("delete_contact", "tbl_Contacts", [], ["id"]),
        ("add_address", "tbl_Address", AddressColumns, []),
        ("update_address", "tbl_Address", AddressColumns, ["id"]),
        ("delete_address", "tbl_Address", [], ["id"]),
        ("add_phone", "tbl_Phone", PhoneColumns, []),
        ("update_phone", "tbl_Phone", PhoneColumns, ["id"]),
        ("delete_phone", "tbl_Phone", [], ["id"]),
        ("add_contact_group_member", "tbl_ContactGroupMember", MemberColumns, []),
        ("delete_contact_group_member", "tbl_ContactGroupMember", [], ["contact_id", "contact_group_id"]),
        ("add_legacy_contact", "legacy_contact", LegacyColumns, []),
        ("update_legacy_contact", "legacy_contact", LegacyColumns, ["key"]),
        ("delete_legacy_contact", "legacy_contact", [], ["key"])
    ];

    static readonly string[] ContactColumns =
    [
        "contact_source_id", "contact_type_id", "created", "email", "first_name", "last_name", "notes", "updated", "uuid", "version"
    ];

    static readonly string[] AddressColumns =
    [
        "address_type_id", "city", "contact_id", "country", "created", "line1", "line2", "postal_code", "region", "updated", "uuid", "version"
    ];

    static readonly string[] PhoneColumns =
    [
        "contact_id", "created", "number", "phone_type_id", "updated", "uuid", "version"
    ];

    static readonly string[] MemberColumns =
    [
        "contact_group_id", "contact_id", "created", "updated", "uuid", "version"
    ];

    static readonly string[] LegacyColumns =
    [
        "email", "first_name", "imported_at", "key", "last_name"
    ];
}
