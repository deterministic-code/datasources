namespace Datasources.Tests.Contact;

internal static class ContactReads
{
    public static NestedSelectStatement All() =>
        new NestedSelectBuilder().Build(ContactGraph());

    public static CreateNestedReadStoredProcedureStatement WithoutPhones(string emailType) =>
        new(
            "ReadContactsByEmail",
            new NestedSelectBuilder().Build(ContactGraph(), new NestedSelectBuilderOptions { StopAt = ["Phones"] }),
            [new NestedWhereArgument("Contact.Email", new RoutineParameter("email", emailType))]);

    public static NestedSelectType ContactGraph()
    {
        var source = new NestedSelectType { Name = "ContactSource", Key = "Id", Properties = ["Id", "Name"] };
        var type = new NestedSelectType { Name = "ContactType", Key = "Id", Properties = ["Id", "Name"] };
        var addressType = new NestedSelectType { Name = "AddressType", Key = "Id", Properties = ["Id", "Name"] };
        var phoneType = new NestedSelectType { Name = "PhoneType", Key = "Id", Properties = ["Id", "Name"] };
        var address = new NestedSelectType
        {
            Name = "Address",
            Key = "Id",
            Properties = ["Id", "Line1", "City"],
            Relations =
            [
                new() { Name = "AddressType", Kind = NestedSelectRelationKind.Object, From = "AddressTypeId", To = "Id", Target = addressType },
            ],
        };
        var phone = new NestedSelectType
        {
            Name = "Phone",
            Key = "Id",
            Properties = ["Id", "Number"],
            Relations =
            [
                new() { Name = "PhoneType", Kind = NestedSelectRelationKind.Object, From = "PhoneTypeId", To = "Id", Target = phoneType },
            ],
        };
        return new NestedSelectType
        {
            Name = "Contact",
            Key = "Id",
            Properties = ["Id", "FirstName", "LastName", "Email"],
            Relations =
            [
                new() { Name = "ContactSource", Kind = NestedSelectRelationKind.Object, From = "ContactSourceId", To = "Id", Target = source },
                new() { Name = "ContactType", Kind = NestedSelectRelationKind.Object, From = "ContactTypeId", To = "Id", Target = type },
                new() { Name = "Addresses", Kind = NestedSelectRelationKind.List, From = "Id", To = "ContactId", Target = address },
                new() { Name = "Phones", Kind = NestedSelectRelationKind.List, From = "Id", To = "ContactId", Target = phone },
            ],
        };
    }
}
