using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactColumns
{
    public static readonly IMapper<string, string> Snake = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);

    static readonly DictionaryMapper<string> References = new(new Dictionary<string, string>
    {
        ["Contact"] = "contact_id",
        ["AddressType"] = "address_type_id",
        ["ContactType"] = "contact_type_id",
        ["ContactSource"] = "contact_source_id",
        ["PhoneType"] = "phone_type_id",
    });

    public static readonly IMapper<string, string> Columns = new TransformMapper<string, string>(
        name => Snake.Map(References.Map(name)),
        name => References.ReverseMap(name) is var from && from != name ? from : Snake.ReverseMap(name));

    public static readonly IMapper<string, string> ContactEntity = new DictionaryMapper<string>(
        new Dictionary<string, string> { ["Contact"] = "contacts_base" });
}
