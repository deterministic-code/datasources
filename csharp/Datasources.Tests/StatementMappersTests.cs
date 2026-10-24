using Datasources;

namespace Datasources.Tests;

public class StatementMappersTests
{
    [Fact]
    public void MapsSelect()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["User"] = "people", ["Role"] = "roles" }),
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Name"] = "full_name", ["title"] = "book.title", ["id"] = "user_id" }));
        Value Mapper(string name, Value value) =>
            value is Value.Text text ? $"{name}:{text.Item.ToUpperInvariant()}" : value;

        var select = new SelectStatement(new SelectStatementOptions
        {
            Distinct = true,
            Table = "User",
            Fields = ["Name", "title", "scope.title", "alias.other", "\"schema.name\".id"],
            Where = new Where(new Operator.Equals("Name", "Bea")),
            Joins = [Join.Inner("Role", "User.id", "Role.user_id"), Join.Cross("logs")],
            GroupBy = ["Name"],
            Having = new Where(new Operator.Equals("Name", "Bea")),
            OrderBy = [OrderBy.Desc("Name")],
            Limit = 5,
            Offset = 1,
        });
        var mapped = Assert.IsType<SelectStatement>(mappings.Apply(select, Mapper));
        Assert.Equal("people", mapped.Table);
        Assert.Equal(["full_name", "book.title", "book.title AS title", "alias.other AS other", "\"schema.name\".user_id AS id"], mapped.Fields);
        Assert.True(mapped.Distinct);
        Assert.Equal(5, mapped.Limit);
        Assert.Equal(1, mapped.Offset);
        Assert.Equal("full_name", Assert.Single(mapped.GroupBy));
        Assert.Equal("full_name", Assert.Single(mapped.OrderBy).Column);
        Assert.Equal(SortDirection.Desc, Assert.Single(mapped.OrderBy).Direction);
        Assert.Equal("people.user_id", ((Operator.Equals)((Join)mapped.Joins[0]).On!).Left is Identifier left ? left.Name : "");
        Assert.Equal("logs", mapped.Joins[1].Table);
        Assert.Null(mapped.Joins[1].On);
        Assert.Contains("full_name = $full_name", DefaultStatementTextRenderer.Instance.Render(mapped));
        Assert.Equal(new Value.Text("Name:BEA"), Assert.Single(mapped.Where!.Parameters).Value);
        Assert.Equal("Name", mappings.Property("full_name"));
        Assert.Equal("Name", mappings.Property("people.full_name"));
        Assert.Equal("other", mappings.Property("other"));
        Assert.Equal("people.missing", mappings.Property("people.missing"));
    }

    [Fact]
    public void MapsWrites()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["User"] = "people", ["Role"] = "roles" }),
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Name"] = "full_name", ["title"] = "book.title", ["id"] = "user_id" }));
        Value Mapper(string name, Value value) =>
            value is Value.Text text ? $"{name}:{text.Item.ToUpperInvariant()}" : value;
        var insert = Assert.IsType<InsertStatement>(mappings.Apply(
            new InsertStatement("User", [new Parameter("Name", "Ada")]), Mapper));
        Assert.Equal("INSERT INTO people (full_name) VALUES ($full_name)", DefaultStatementTextRenderer.Instance.Render(insert));
        Assert.Equal(new Value.Text("Ada"), Assert.Single(insert.Fields).Value);

        var update = Assert.IsType<UpdateStatement>(mappings.Apply(
            new UpdateStatement("User", [new Parameter("Name", "Ada")], new Where(new Operator.Equals("Name", "Bea"))),
            Mapper));
        Assert.Equal(
            "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where",
            DefaultStatementTextRenderer.Instance.Render(update));
        Assert.Equal(new Value.Text("Ada"), update.Parameters[0].Value);
        Assert.Equal("full_name__where", update.Parameters[1].Name);
        Assert.Equal(new Value.Text("Name:BEA"), update.Parameters[1].Value);

        var delete = Assert.IsType<DeleteStatement>(mappings.Apply(
            new DeleteStatement("User", new Where(new Operator.Equals("Name", "Bea"))), Mapper));
        Assert.Equal("DELETE FROM people WHERE full_name = $full_name", DefaultStatementTextRenderer.Instance.Render(delete));

        var inserted = Assert.IsType<SelectInsertedIdStatement>(mappings.Apply(
            new SelectInsertedIdStatement("User", "id"), Mapper));
        Assert.Equal("people", inserted.Table);
        Assert.Equal("user_id", inserted.Column);
    }

    [Fact]
    public void LeavesRawAndOtherStatements()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["User"] = "people", ["Role"] = "roles" }),
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Name"] = "full_name", ["title"] = "book.title", ["id"] = "user_id" }));
        Value Mapper(string name, Value value) => value;
        var call = new ExecuteStoredProcedureStatement("save_user", []);
        Assert.Same(call, mappings.Apply(call, Mapper));
        Assert.Null(mappings.Apply((IStatement?)null, Mapper));
        Assert.Null(mappings.Apply((IWhere?)null, Mapper));
        var text = new WhereText("name = $name");
        Assert.Same(text, mappings.Apply(text, Mapper));
        var kept = Assert.IsType<DeleteStatement>(mappings.Apply(new DeleteStatement("User", text), Mapper));
        Assert.Same(text, kept.Where);
        Assert.Equal("DELETE FROM people WHERE name = $name", DefaultStatementTextRenderer.Instance.Render(kept));
    }

    [Fact]
    public void MapsSelectFieldAlias()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts_base", ["Address"] = "address" }),
            new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Id"] = "id",
                ["FirstName"] = "first_name",
                ["ContactId"] = "contact_id",
            }));
        Value Mapper(string name, Value value) => value;

        var select = new SelectStatement(new SelectStatementOptions
        {
            Table = "Contact",
            Fields = ["Contact.Id", "Contact.FirstName", "Address.Id"],
            Joins = [Join.Left("Address", "Contact.Id", "Address.ContactId")],
        });
        var mapped = Assert.IsType<SelectStatement>(mappings.Apply(select, Mapper));
        Assert.Equal(
            "SELECT contacts_base.id AS Id, contacts_base.first_name AS FirstName, address.id AS AddressId FROM contacts_base LEFT JOIN address ON contacts_base.id = address.contact_id",
            DefaultStatementTextRenderer.Instance.Render(mapped));
    }

    [Fact]
    public void MapsContactsSelect()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Contact"] = "contacts_base",
                ["ContactSource"] = "contact_source",
                ["ContactType"] = "contact_type",
                ["Address"] = "address",
                ["AddressType"] = "address_type",
                ["Phone"] = "phone",
                ["PhoneType"] = "phone_type",
            }),
            new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Id"] = "id",
                ["FirstName"] = "first_name",
                ["LastName"] = "last_name",
                ["Email"] = "email",
                ["Name"] = "name",
                ["Line1"] = "line1",
                ["City"] = "city",
                ["Number"] = "number",
                ["ContactId"] = "contact_id",
                ["ContactSourceId"] = "contact_source_id",
                ["ContactTypeId"] = "contact_type_id",
                ["AddressTypeId"] = "address_type_id",
                ["PhoneTypeId"] = "phone_type_id",
            }));
        Value Mapper(string name, Value value) =>
            value is Value.Text text ? $"{name}:{text.Item.ToUpperInvariant()}" : value;

        var select = new SelectStatement(new SelectStatementOptions
        {
            Distinct = true,
            Table = "Contact",
            Fields =
            [
                "Contact.FirstName",
                "Contact.LastName",
                "Contact.Email",
                "ContactSource.Name",
                "Address.Line1",
                "Address.City",
                "Phone.Number",
            ],
            Where = new Where(new Operator.And(
                new Operator.Equals("Contact.Email", "ada@ex.com"),
                new Operator.GreaterThan("Address.City", "A"))),
            Joins =
            [
                Join.Inner("ContactSource", "Contact.ContactSourceId", "ContactSource.Id"),
                Join.Inner("ContactType", "Contact.ContactTypeId", "ContactType.Id"),
                Join.Left("Address", "Contact.Id", "Address.ContactId"),
                Join.Left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
                Join.Left("Phone", "Contact.Id", "Phone.ContactId"),
                Join.Left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
            ],
            GroupBy = ["Contact.LastName"],
            Having = new Where(new Operator.Equals("ContactType.Name", "person")),
            OrderBy = [OrderBy.Asc("Contact.LastName"), OrderBy.Desc("Contact.FirstName")],
            Limit = 20,
            Offset = 1,
        });

        var mapped = Assert.IsType<SelectStatement>(mappings.Apply(select, Mapper));
        Assert.Equal(
            "SELECT DISTINCT contacts_base.first_name AS FirstName, contacts_base.last_name AS LastName, contacts_base.email AS Email, contact_source.name AS Name, address.line1 AS Line1, address.city AS City, phone.number AS Number FROM contacts_base INNER JOIN contact_source ON contacts_base.contact_source_id = contact_source.id INNER JOIN contact_type ON contacts_base.contact_type_id = contact_type.id LEFT JOIN address ON contacts_base.id = address.contact_id LEFT JOIN address_type ON address.address_type_id = address_type.id LEFT JOIN phone ON contacts_base.id = phone.contact_id LEFT JOIN phone_type ON phone.phone_type_id = phone_type.id WHERE contacts_base.email = $contacts_base.email AND address.city > $address.city GROUP BY contacts_base.last_name HAVING contact_type.name = $contact_type.name ORDER BY contacts_base.last_name ASC, contacts_base.first_name DESC LIMIT 20 OFFSET 1",
            DefaultStatementTextRenderer.Instance.Render(mapped));
        Assert.Equal(new Value.Text("Contact.Email:ADA@EX.COM"), mapped.Where!.Parameters[0].Value);
        Assert.Equal(new Value.Text("Address.City:A"), mapped.Where.Parameters[1].Value);
        Assert.Equal(new Value.Text("ContactType.Name:PERSON"), Assert.Single(mapped.Having!.Parameters).Value);
    }

    [Fact]
    public void RejectsSelectFieldAlias()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts_base" }),
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Id"] = "id" }));
        var select = new SelectStatement(new SelectStatementOptions
        {
            Table = "Contact",
            Fields = ["Contact.Id AS Id"],
        });
        var error = Assert.Throws<DatasourceException>(() => mappings.Apply(select, (_, value) => value));
        Assert.Equal("Select field \"Contact.Id AS Id\" must be a path", error.Message);
    }

    [Fact]
    public void MapsAggregateAliasFromLookup()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts_base" }),
            new DictionaryMapper<string>(new Dictionary<string, string> { ["Id"] = "id" }));
        var select = new SelectStatement(new SelectStatementOptions
        {
            Table = "Contact",
            Fields = [Aggregate.Count("Contact.Id")],
            Aliases = ["n"],
        });
        var mapped = Assert.IsType<SelectStatement>(mappings.Apply(select, (_, value) => value));
        Assert.Equal(["COUNT(contacts_base.id) AS n"], mapped.Fields);
    }

    [Fact]
    public void MapsNestedPathAlias()
    {
        var mappings = new StatementMappers(
            new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Contact"] = "contacts_base",
                ["Address"] = "address",
                ["AddressType"] = "address_type",
                ["ContactSource"] = "contact_source",
            }),
            new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Id"] = "id",
                ["City"] = "city",
                ["Name"] = "name",
            }));
        var select = new SelectStatement(new SelectStatementOptions
        {
            Table = "Contact",
            Fields =
            [
                "Contact.Id",
                "Contact.Address.Id",
                "Contact.Address.City",
                "Contact.ContactSource.Name",
                "Contact.Address.AddressType.Name",
            ],
        });
        var mapped = Assert.IsType<SelectStatement>(mappings.Apply(select, (_, value) => value));
        Assert.Equal(
            ["contacts_base.id AS Id", "address.id AS AddressId", "address.city AS City", "contact_source.name AS Name", "address_type.name AS AddressTypeName"],
            mapped.Fields);
    }
}
