using System.Text.Json;
using Backend.Types.View;
using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class Api
{
    static readonly JsonSerializerOptions Json = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        PropertyNameCaseInsensitive = true,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    readonly ContactRepository contacts;
    readonly LegacyContactRepository legacy;

    public Api(ContactRepository contacts, LegacyContactRepository legacy)
    {
        this.contacts = contacts;
        this.legacy = legacy;
    }

    public async Task<(int Status, string Body)> Handle(string method, string target, string? body)
    {
        try
        {
            var split = target.IndexOf('?');
            var raw = (split < 0 ? target : target[..split]).TrimEnd('/');
            var path = raw.Length == 0 ? "/" : raw;
            var query = Parse(split < 0 ? "" : target[(split + 1)..]);
            return await ($"{method.ToUpperInvariant()} {path}" switch
            {
                "GET /contacts" => GetContacts(query),
                "POST /contacts" => PostContact(body),
                "PUT /contacts" => PutContact(query, body),
                "DELETE /contacts" => DeleteContact(query),
                "GET /legacy_contacts" => GetLegacy(query),
                "POST /legacy_contacts" => PostLegacy(body),
                "PUT /legacy_contacts" => PutLegacy(query, body),
                "DELETE /legacy_contacts" => DeleteLegacy(query),
                _ => Task.FromResult((404, "")),
            });
        }
        catch (RequestError error)
        {
            return (error.Status, error.Message);
        }
        catch (Exception error) when (error is DatasourceException or JsonException or ArgumentException)
        {
            return (400, error.Message);
        }
    }

    async Task<(int Status, string Body)> GetContacts(Dictionary<string, string> query) =>
        (200, JsonSerializer.Serialize(await List(contacts, WhereOf(query, ["Id", "FirstName", "LastName", "Email"])), Json));

    async Task<(int Status, string Body)> PostContact(string? body)
    {
        var contact = await Bind(Read<Contact>(body));
        return (201, JsonSerializer.Serialize(await contacts.Create(contact), Json));
    }

    async Task<(int Status, string Body)> PutContact(Dictionary<string, string> query, string? body)
    {
        var id = RequiredNumber(query, "id");
        if (await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id))) is null)
            return (404, "");
        var contact = await Bind(Read<Contact>(body));
        contact.Id = id;
        await contacts.Update(contact);
        return (200, JsonSerializer.Serialize(await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id))), Json));
    }

    async Task<(int Status, string Body)> DeleteContact(Dictionary<string, string> query)
    {
        var id = RequiredNumber(query, "id");
        var contact = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id)));
        if (contact is null) return (404, "");
        await contacts.Delete(contact);
        return (204, "");
    }

    async Task<(int Status, string Body)> GetLegacy(Dictionary<string, string> query) =>
        (200, JsonSerializer.Serialize(await List(legacy, WhereOf(query, ["Key", "FirstName", "LastName", "Email"])), Json));

    async Task<(int Status, string Body)> PostLegacy(string? body)
    {
        var row = TextLegacy(Read<LegacyContact>(body));
        if (row.Key.Length == 0) throw new RequestError("key is required");
        return (201, JsonSerializer.Serialize(await legacy.Create(row), Json));
    }

    async Task<(int Status, string Body)> PutLegacy(Dictionary<string, string> query, string? body)
    {
        var key = Required(query, "key");
        if (await legacy.ReadSingle(new Where(new Datasources.Operator.Equals("Key", key))) is null)
            return (404, "");
        var row = TextLegacy(Read<LegacyContact>(body));
        row.Key = key;
        await legacy.Update(row, new Where(new Datasources.Operator.Equals("Key", key)));
        return (200, JsonSerializer.Serialize(await legacy.ReadSingle(new Where(new Datasources.Operator.Equals("Key", key))), Json));
    }

    async Task<(int Status, string Body)> DeleteLegacy(Dictionary<string, string> query)
    {
        var key = Required(query, "key");
        var row = await legacy.ReadSingle(new Where(new Datasources.Operator.Equals("Key", key)));
        if (row is null) return (404, "");
        await legacy.Delete(row, new Where(new Datasources.Operator.Equals("Key", key)));
        return (204, "");
    }

    async Task<Contact> Bind(Contact contact)
    {
        contact.ContactSourceId = await LookupId(contacts.ContactSource, contact.ContactSourceName, "contact_source");
        contact.Email = contact.Email ?? "";
        contact.Notes = contact.Notes ?? "";
        contact.Uuid = string.IsNullOrEmpty(contact.Uuid) ? Clock.Uuid() : contact.Uuid;
        contact.Created = contact.Created == default ? Clock.Now() : contact.Created;
        contact.Updated = Clock.Now();
        contact.Version ??= [0];
        contact.Addresses ??= [];
        contact.Phones ??= [];
        contact.Settings ??= new Dictionary<string, string>();
        contact.Prefs ??= new Dictionary<string, LocalePreferences>();
        foreach (var address in contact.Addresses)
        {
            address.AddressTypeId = await LookupId(contacts.Address.AddressType, address.AddressTypeName, "address_type");
            address.Line1 = address.Line1 ?? "";
            address.Line2 = address.Line2 ?? "";
            address.Region = address.Region ?? "";
            address.PostalCode = address.PostalCode ?? "";
            address.Country = address.Country ?? "";
            address.Uuid = string.IsNullOrEmpty(address.Uuid) ? Clock.Uuid() : address.Uuid;
            address.Created = address.Created == default ? Clock.Now() : address.Created;
            address.Updated = Clock.Now();
            address.Version ??= [0];
        }
        foreach (var phone in contact.Phones)
        {
            phone.PhoneTypeId = await LookupId(contacts.Phone.PhoneType, phone.PhoneTypeName, "phone_type");
            phone.Number = phone.Number ?? "";
            phone.Label = phone.Label ?? "";
            phone.Uuid = string.IsNullOrEmpty(phone.Uuid) ? Clock.Uuid() : phone.Uuid;
            phone.Created = phone.Created == default ? Clock.Now() : phone.Created;
            phone.Updated = Clock.Now();
            phone.Version ??= [0];
        }
        return contact;
    }

    async Task<long> LookupId(IReadonlyRepository<Backend.Types.Datasource.ContactSource> repo, string? name, string label)
    {
        if (string.IsNullOrWhiteSpace(name)) throw new RequestError($"{label} is required");
        var found = await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Name", name)));
        return found is null ? throw new RequestError($"Unknown {label} \"{name}\"") : found.Id;
    }

    async Task<long> LookupId(IReadonlyRepository<Backend.Types.Datasource.AddressType> repo, string? name, string label)
    {
        if (string.IsNullOrWhiteSpace(name)) throw new RequestError($"{label} is required");
        var found = await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Name", name)));
        return found is null ? throw new RequestError($"Unknown {label} \"{name}\"") : found.Id;
    }

    async Task<long> LookupId(IReadonlyRepository<Backend.Types.Datasource.PhoneType> repo, string? name, string label)
    {
        if (string.IsNullOrWhiteSpace(name)) throw new RequestError($"{label} is required");
        var found = await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Name", name)));
        return found is null ? throw new RequestError($"Unknown {label} \"{name}\"") : found.Id;
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo, IWhere? query)
    {
        var rows = new List<T>();
        await foreach (var row in query is null ? repo.Read() : repo.Read(query)) rows.Add(row);
        return rows;
    }

    static IWhere? WhereOf(Dictionary<string, string> query, string[] keys)
    {
        IExpression? combined = null;
        foreach (var key in keys.Where(key => query.GetValueOrDefault(ToCamel(key), "").Length > 0))
        {
            IExpression part = new Datasources.Operator.Equals(key, key == "Id" ? long.Parse(query[ToCamel(key)]) : (Value)query[ToCamel(key)]);
            combined = combined is null ? part : new Datasources.Operator.And(combined, part);
        }
        return combined is null ? null : new Where(combined);
    }

    static string ToCamel(string name) => char.ToLowerInvariant(name[0]) + name[1..];

    static T Read<T>(string? body) where T : new()
    {
        if (string.IsNullOrWhiteSpace(body)) return new T();
        return JsonSerializer.Deserialize<T>(body, Json) ?? throw new JsonException("invalid json");
    }

    static LegacyContact TextLegacy(LegacyContact row)
    {
        row.Key = row.Key ?? "";
        row.FirstName = row.FirstName ?? "";
        row.LastName = row.LastName ?? "";
        row.Email = row.Email ?? "";
        if (row.ImportedAt == default) row.ImportedAt = Clock.Now();
        return row;
    }

    static long RequiredNumber(Dictionary<string, string> query, string key) =>
        long.TryParse(Required(query, key), out var value) ? value : throw new RequestError($"{key} is required");

    static string Required(Dictionary<string, string> query, string key) =>
        (query.GetValueOrDefault(key) ?? "").Length > 0 ? query[key] : throw new RequestError($"{key} is required");

    static Dictionary<string, string> Parse(string query) =>
        query.Split('&', StringSplitOptions.RemoveEmptyEntries)
            .Select(part =>
            {
                var eq = part.IndexOf('=');
                return (
                    Uri.UnescapeDataString(eq < 0 ? part : part[..eq]),
                    Uri.UnescapeDataString(eq < 0 ? "" : part[(eq + 1)..]));
            })
            .ToDictionary(pair => pair.Item1, pair => pair.Item2);

    sealed class RequestError(string message, int status = 400) : Exception(message)
    {
        public int Status { get; } = status;
    }
}
