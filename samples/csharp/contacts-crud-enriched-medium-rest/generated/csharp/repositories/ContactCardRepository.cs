using Datasources;

public sealed class ContactCardRepository : CRUDRepository<Backend.Types.View.ContactCard>
{
    public ContactCardRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.ContactCard> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.ContactCard>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.ContactCard> Options() => new()
    {
        Entity = "ContactCard",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactCard"] = "contact_cards" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["DisplayName"] = "display_name",
        }),
        Ids = [],
        ReadProperties = ["DisplayName"],
        CreateProperties = ["DisplayName"],
        UpdateProperties = ["DisplayName"],
    };
}
