namespace Datasources.Tests.Contact;

internal static class SeedContacts
{
    public const int Count = 250;

    public static IReadOnlyList<string> Sql(int count = Count, string version = "X''")
    {
        const string stamp = "2024-01-01T00:00:00Z";
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
                statements.Add(
                    $"""INSERT INTO "address" ("id", "uuid", "created", "updated", "version", "contact_id", "line1", "line2", "city", "region", "postal_code", "country") VALUES ({addressId}, '{Uuid(1000 + addressId)}', '{stamp}', '{stamp}', {version}, {id}, '{id} Line {a + 1}', NULL, 'City{id}', NULL, NULL, NULL)""");
                addressId += 1;
            }
            for (var p = 0; p < id % 3; p++)
            {
                var label = p == 0 ? "mobile" : "work";
                statements.Add(
                    $"""INSERT INTO "phone" ("id", "uuid", "created", "updated", "version", "contact_id", "number", "label") VALUES ({phoneId}, '{Uuid(2000 + phoneId)}', '{stamp}', '{stamp}', {version}, {id}, '555-000-{id.ToString().PadLeft(4, '0')}-{p}', '{label}')""");
                phoneId += 1;
            }
        }
        return statements;
    }
}
