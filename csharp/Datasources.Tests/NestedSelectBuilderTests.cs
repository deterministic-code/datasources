using Datasources.Tests.Contact;

namespace Datasources.Tests;

public class NestedSelectBuilderTests
{
    [Fact]
    public void BuildsContactJoinsAndMap()
    {
        var nested = ContactReads.All();
        Assert.Equal("Contact", nested.Select.Table);
        Assert.Equal(
            [
                "Contact.Id",
                "Contact.FirstName",
                "Contact.LastName",
                "Contact.Email",
                "Contact.ContactSource.Id",
                "Contact.ContactSource.Name",
                "Contact.ContactType.Id",
                "Contact.ContactType.Name",
                "Contact.Address.Id",
                "Contact.Address.Line1",
                "Contact.Address.City",
                "Contact.Address.AddressType.Id",
                "Contact.Address.AddressType.Name",
                "Contact.Phone.Id",
                "Contact.Phone.Number",
                "Contact.Phone.PhoneType.Id",
                "Contact.Phone.PhoneType.Name",
            ],
            nested.Select.Fields);
        Assert.Equal(6, nested.Select.Joins.Count);
        Assert.Equal("ContactSource", nested.Select.Joins[0].Table);
        Assert.Equal("PhoneType", nested.Select.Joins[5].Table);
        Assert.Equal("Id", nested.Map.Key);
        Assert.Contains("FirstName", nested.Map.Fields.Keys);
        Assert.True(nested.Map.Fields["Addresses"] is TypeMapField.Group group && group.Map.Key == "AddressId");
        Assert.True(nested.Map.Fields["Phones"] is TypeMapField.Group phones && phones.Map.Key == "PhoneId");
    }

    [Fact]
    public void StopAtDropsPhones()
    {
        var nested = new NestedSelectBuilder().Build(ContactReads.ContactGraph(), new NestedSelectBuilderOptions { StopAt = ["Phones"] });
        Assert.DoesNotContain(nested.Select.Fields, field => field.Contains("Phone", StringComparison.Ordinal));
        Assert.DoesNotContain(nested.Select.Joins, join => join.Table.Contains("Phone", StringComparison.Ordinal));
        Assert.False(nested.Map.Fields.ContainsKey("Phones"));
        Assert.True(nested.Map.Fields.ContainsKey("Addresses"));
    }

    [Fact]
    public void ProcedureAddsWhereArgument()
    {
        var procedure = ContactReads.WithoutPhones("varchar");
        Assert.Equal("ReadContactsByEmail", procedure.Name);
        Assert.Equal("Contact.Email", Assert.Single(procedure.Arguments).Path);
        Assert.Contains("Contact.Email", DefaultStatementTextRenderer.Instance.Render(procedure.Select));
        Assert.DoesNotContain("Phone", DefaultStatementTextRenderer.Instance.Render(procedure.Select), StringComparison.Ordinal);
        Assert.Equal("", DefaultStatementTextRenderer.Instance.Render(procedure));
    }

    [Fact]
    public void ProcedureAndsOntoExistingWhere()
    {
        var nested = new NestedSelectBuilder().Build(
            ContactReads.ContactGraph(),
            new NestedSelectBuilderOptions
            {
                StopAt = ["Phones"],
                Where = new Where(new Operator.Equals("Contact.Id", 1)),
            });
        var procedure = new CreateNestedReadStoredProcedureStatement(
            "ReadContactsByEmail",
            nested,
            [new NestedWhereArgument("Contact.Email", new RoutineParameter("email", "varchar"))]);
        Assert.Contains(" AND ", DefaultStatementTextRenderer.Instance.Render(procedure.Select));
    }
}
