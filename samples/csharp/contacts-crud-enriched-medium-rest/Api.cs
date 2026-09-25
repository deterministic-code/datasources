using System.Text.Json;
using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class Api(ContactsRepository contacts, LegacyContactRepository legacy)
{
    static readonly JsonSerializerOptions Json = new() { PropertyNameCaseInsensitive = true };

    public async Task<HttpResult> Handle(string method, string target, string? body)
    {
        try
        {
            var split = target.IndexOf('?');
            var path = (split < 0 ? target : target[..split]).TrimEnd('/');
            var query = Parse(split < 0 ? "" : target[(split + 1)..]);
            return (method.ToUpperInvariant(), path) switch
            {
                ("GET", "/contacts") => await GetContacts(query),
                ("POST", "/contacts") => await PostContact(body),
                ("PUT", "/contacts") => await PutContact(query, body),
                ("DELETE", "/contacts") => await DeleteContact(query),
                ("GET", "/legacy_contacts") => await GetLegacy(query),
                ("POST", "/legacy_contacts") => await PostLegacy(body),
                ("PUT", "/legacy_contacts") => await PutLegacy(query, body),
                ("DELETE", "/legacy_contacts") => await DeleteLegacy(query),
                _ => new HttpResult(404, "")
            };
        }
        catch (RequestException ex)
        {
            return new HttpResult(ex.Status, ex.Message);
        }
        catch (JsonException ex)
        {
            return new HttpResult(400, ex.Message);
        }
        catch (DatasourceException ex)
        {
            return new HttpResult(400, ex.Message);
        }
        catch (FormatException ex)
        {
            return new HttpResult(400, ex.Message);
        }
    }

    async Task<HttpResult> GetContacts(Dictionary<string, string> query) =>
        new(200, JsonSerializer.Serialize(await List(contacts, Where(query, "first_name", "last_name", "email", "id")), Json));

    async Task<HttpResult> PostContact(string? body)
    {
        var contact = Read<ContactsView>(body);
        await Bind(contact);
        return new HttpResult(201, JsonSerializer.Serialize(await contacts.Create(contact), Json));
    }

    async Task<HttpResult> PutContact(Dictionary<string, string> query, string? body)
    {
        var id = RequiredInt(query, "id");
        if (await contacts.ReadSingle(new WhereClause(new Datasources.Operator.Equals("id", id))) is null)
            return new HttpResult(404, "");
        var contact = Read<ContactsView>(body);
        contact.id = id;
        await Bind(contact);
        await contacts.Update(contact);
        return new HttpResult(200, JsonSerializer.Serialize(await contacts.ReadSingle(new WhereClause(new Datasources.Operator.Equals("id", id))), Json));
    }

    async Task<HttpResult> DeleteContact(Dictionary<string, string> query)
    {
        var id = RequiredInt(query, "id");
        var contact = await contacts.ReadSingle(new WhereClause(new Datasources.Operator.Equals("id", id)));
        if (contact is null)
            return new HttpResult(404, "");
        await contacts.Delete(contact);
        return new HttpResult(204, "");
    }

    async Task<HttpResult> GetLegacy(Dictionary<string, string> query) =>
        new(200, JsonSerializer.Serialize(await List(legacy, Where(query, "key", "first_name", "last_name", "email")), Json));

    async Task<HttpResult> PostLegacy(string? body)
    {
        var row = Text(Read<LegacyContact>(body));
        if (row.key.Length == 0)
            throw new RequestException("key is required");
        return new HttpResult(201, JsonSerializer.Serialize(await legacy.Create(row), Json));
    }

    async Task<HttpResult> PutLegacy(Dictionary<string, string> query, string? body)
    {
        var key = Required(query, "key");
        if (await legacy.ReadSingle(new WhereClause(new Datasources.Operator.Equals("key", key))) is null)
            return new HttpResult(404, "");
        var row = Text(Read<LegacyContact>(body));
        row.key = key;
        await legacy.Update(row, new WhereClause(new Datasources.Operator.Equals("key", key)));
        return new HttpResult(200, JsonSerializer.Serialize(await legacy.ReadSingle(new WhereClause(new Datasources.Operator.Equals("key", key))), Json));
    }

    async Task<HttpResult> DeleteLegacy(Dictionary<string, string> query)
    {
        var key = Required(query, "key");
        var row = await legacy.ReadSingle(new WhereClause(new Datasources.Operator.Equals("key", key)));
        if (row is null)
            return new HttpResult(404, "");
        await legacy.Delete(row, new WhereClause(new Datasources.Operator.Equals("key", key)));
        return new HttpResult(204, "");
    }

    async Task Bind(ContactsView contact)
    {
        contact.contact_source_id = await LookupId(contacts.Sources, "contact_source", contact.contact_source);
        contact.contact_type_id = await LookupId(contacts.Types, "contact_type", contact.contact_type);
        contact.email = Text(contact.email);
        contact.notes = Text(contact.notes);
        Stamp(contact);
        foreach (var address in contact.addresses)
        {
            address.address_type_id = await LookupId(contacts.AddressTypes, "address_type", address.address_type);
            address.line1 = Text(address.line1);
            address.line2 = Text(address.line2);
            address.city = Text(address.city);
            address.region = Text(address.region);
            address.postal_code = Text(address.postal_code);
            address.country = Text(address.country);
            Stamp(address);
        }
        foreach (var phone in contact.phones)
        {
            phone.phone_type_id = await LookupId(contacts.PhoneTypes, "phone_type", phone.phone_type);
            phone.number = Text(phone.number);
            Stamp(phone);
        }
    }

    async Task<int> LookupId(CRUDRepository<Lookup> repo, string field, string name)
    {
        if (string.IsNullOrWhiteSpace(name))
            throw new RequestException($"{field} is required");
        var found = await repo.ReadSingle(new WhereClause(new Datasources.Operator.Equals("name", name)));
        return found?.id ?? throw new RequestException($"Unknown {field} \"{name}\"");
    }

    static void Stamp(Contact contact)
    {
        contact.uuid = contact.uuid == Guid.Empty ? Guid.NewGuid() : contact.uuid;
        contact.created = contact.created == default ? DateTime.UtcNow : contact.created;
        contact.updated = DateTime.UtcNow;
        contact.version = [0];
    }

    static void Stamp(Address address)
    {
        address.uuid = address.uuid == Guid.Empty ? Guid.NewGuid() : address.uuid;
        address.created = address.created == default ? DateTime.UtcNow : address.created;
        address.updated = DateTime.UtcNow;
        address.version = [0];
    }

    static void Stamp(Phone phone)
    {
        phone.uuid = phone.uuid == Guid.Empty ? Guid.NewGuid() : phone.uuid;
        phone.created = phone.created == default ? DateTime.UtcNow : phone.created;
        phone.updated = DateTime.UtcNow;
        phone.version = [0];
    }

    static async Task<List<T>> List<T>(CRUDRepository<T> repo, IWhere? where) where T : new()
    {
        var rows = new List<T>();
        if (where is null)
        {
            await foreach (var row in repo.Read())
                rows.Add(row);
            return rows;
        }
        await foreach (var row in repo.Read(where))
            rows.Add(row);
        return rows;
    }

    static IWhere? Where(Dictionary<string, string> query, params string[] keys)
    {
        IExpression? expr = null;
        foreach (var key in keys)
        {
            if (!query.TryGetValue(key, out var value) || value.Length == 0)
                continue;
            IExpression eq = key == "id"
                ? new Datasources.Operator.Equals(key, int.Parse(value))
                : new Datasources.Operator.Equals(key, value);
            expr = expr is null ? eq : new Datasources.Operator.And(expr, eq);
        }
        return expr is null ? null : new WhereClause(expr);
    }

    static T Read<T>(string? body) where T : new() =>
        string.IsNullOrWhiteSpace(body) ? new T() : JsonSerializer.Deserialize<T>(body, Json) ?? new T();

    static LegacyContact Text(LegacyContact row)
    {
        row.key = Text(row.key);
        row.first_name = Text(row.first_name);
        row.last_name = Text(row.last_name);
        row.email = Text(row.email);
        row.imported_at ??= DateTime.UtcNow;
        return row;
    }

    static string Text(string? value) => value ?? "";

    static int RequiredInt(Dictionary<string, string> query, string key) =>
        int.Parse(Required(query, key));

    static string Required(Dictionary<string, string> query, string key) =>
        query.TryGetValue(key, out var value) && value.Length > 0 ? value : throw new RequestException($"{key} is required");

    static Dictionary<string, string> Parse(string query)
    {
        var result = new Dictionary<string, string>(StringComparer.Ordinal);
        foreach (var part in query.Split('&', StringSplitOptions.RemoveEmptyEntries))
        {
            var eq = part.IndexOf('=');
            var name = Uri.UnescapeDataString(eq < 0 ? part : part[..eq]);
            var value = Uri.UnescapeDataString(eq < 0 ? "" : part[(eq + 1)..]);
            result[name] = value;
        }
        return result;
    }

    sealed class RequestException(string message, int status = 400) : Exception(message)
    {
        public int Status { get; } = status;
    }
}
