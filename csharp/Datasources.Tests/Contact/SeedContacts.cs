namespace Datasources.Tests.Contact;

internal static class SeedContacts
{
    public const int Count = 250;

    public static IReadOnlyList<string> Sql(int count = Count, string version = "X''")
    {
        const string stamp = "2024-01-01 00:00:00";
        static string Uuid(int n) => $"00000000-0000-4000-8000-{n.ToString().PadLeft(12, '0')}";
        var statements = new List<string>();
        var addressId = 1;
        var phoneId = 1;
        for (var id = 1; id <= count; id++)
        {
            var source = (id % 3) + 1;
            var type = (id % 2) + 1;
            statements.Add(
                $"""INSERT INTO "contacts_base" ("id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes") VALUES ({id}, '{Uuid(id)}', '{stamp}', '{stamp}', {version}, {source}, {type}, 'First{id}', 'Last{id}', 'c{id}@ex.com', NULL)""");
            for (var a = 0; a < id % 4; a++)
            {
                var addressType = a == 0 ? 1 : 2;
                statements.Add(
                    $"""INSERT INTO "address" ("id", "uuid", "created", "updated", "version", "contact_id", "address_type_id", "line1", "line2", "city", "region", "postal_code", "country") VALUES ({addressId}, '{Uuid(1000 + addressId)}', '{stamp}', '{stamp}', {version}, {id}, {addressType}, '{id} Line {a + 1}', NULL, 'City{id}', NULL, NULL, NULL)""");
                addressId += 1;
            }
            for (var p = 0; p < id % 3; p++)
            {
                var phoneType = p == 0 ? 1 : 2;
                statements.Add(
                    $"""INSERT INTO "phone" ("id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number") VALUES ({phoneId}, '{Uuid(2000 + phoneId)}', '{stamp}', '{stamp}', {version}, {id}, {phoneType}, '555-000-{id.ToString().PadLeft(4, '0')}-{p}')""");
                phoneId += 1;
            }
        }
        return statements;
    }

    public static IEnumerable<string> IdentitySql(string engine) => engine switch
    {
        "postgres" => new[] { "contacts_base", "address", "phone" }.Select(table =>
            $"SELECT setval(pg_get_serial_sequence('{table}', 'id'), COALESCE((SELECT MAX(id) FROM \"{table}\"), 1))"),
        "mysql" => new[] { "contacts_base", "address", "phone" }.Select(table =>
            $"ALTER TABLE {table} AUTO_INCREMENT = {Count + 1}"),
        _ => [],
    };
}
