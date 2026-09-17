using Datasources;

namespace Datasources.Tests;

public class FlatToTypeMapperTests
{
    static readonly TypeMap ContactsView = new()
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "id",
            ["first_name"] = "first_name",
            ["last_name"] = "last_name",
            ["email"] = "email",
            ["contact_source_name"] = "contact_source_name",
            ["contact_type_name"] = "contact_type_name",
            ["addresses"] = new TypeMapGroup
            {
                Key = "address_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "address_id",
                    ["line1"] = "address_line1",
                    ["city"] = "address_city",
                    ["address_type_name"] = "address_type_name",
                },
            },
            ["phones"] = new TypeMapGroup
            {
                Key = "phone_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "phone_id",
                    ["number"] = "phone_number",
                    ["phone_type_name"] = "phone_type_name",
                },
            },
        },
    };

    static Row AdaHome() => new(
    [
        new("id", 7),
        new("first_name", "Ada"),
        new("last_name", "Lovelace"),
        new("email", "ada@ex.com"),
        new("contact_source_name", "Web"),
        new("contact_type_name", "Person"),
        new("address_id", 101),
        new("address_line1", "10 Downing St"),
        new("address_city", "London"),
        new("address_type_name", "Home"),
        new("phone_id", 201),
        new("phone_number", "+44"),
        new("phone_type_name", "Mobile"),
    ]);

    static Row AdaWork()
    {
        var row = new Row(AdaHome());
        row["address_id"] = 102;
        row["address_line1"] = "1 Parliament Sq";
        return row;
    }

    static Row NoAddress() => new(
    [
        new("id", 8),
        new("first_name", "Charles"),
        new("last_name", "Babbage"),
        new("email", Value.Null.Instance),
        new("contact_source_name", "Import"),
        new("contact_type_name", "Person"),
        new("address_id", Value.Null.Instance),
        new("address_line1", Value.Null.Instance),
        new("address_city", Value.Null.Instance),
        new("address_type_name", Value.Null.Instance),
        new("phone_id", 202),
        new("phone_number", "555"),
        new("phone_type_name", "Work"),
    ]);

    static readonly FlatToTypeMapper Mapper = new(ContactsView);

    static Value Cell(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var value)
            ? value
            : Value.Null.Instance;

    static List<Dictionary<string, MappedValue>> Many(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Many(var items) ? items : [];

    static void AssertRow(Row expected, Row actual)
    {
        Assert.Equal(expected.Count, actual.Count);
        foreach (var (key, value) in expected)
            Assert.Equal(value, actual[key]);
    }

    [Fact]
    public void MapsOneDenormalizedRowIntoAContactWith01Children()
    {
        var home = Mapper.Map(AdaHome());
        Assert.Equal(new Value.Int(7), Cell(home, "id"));
        Assert.Equal(new Value.Text("Ada"), Cell(home, "first_name"));
        Assert.Equal(new Value.Text("Lovelace"), Cell(home, "last_name"));
        Assert.Equal(new Value.Text("ada@ex.com"), Cell(home, "email"));
        Assert.Equal(new Value.Text("Web"), Cell(home, "contact_source_name"));
        Assert.Equal(new Value.Text("Person"), Cell(home, "contact_type_name"));
        Assert.Single(Many(home, "addresses"));
        Assert.Equal(new Value.Int(101), Cell(Many(home, "addresses")[0], "id"));
        Assert.Equal(new Value.Text("10 Downing St"), Cell(Many(home, "addresses")[0], "line1"));
        Assert.Equal(new Value.Text("London"), Cell(Many(home, "addresses")[0], "city"));
        Assert.Equal(new Value.Text("Home"), Cell(Many(home, "addresses")[0], "address_type_name"));
        Assert.Single(Many(home, "phones"));
        Assert.Equal(new Value.Int(201), Cell(Many(home, "phones")[0], "id"));
        Assert.Equal(new Value.Text("+44"), Cell(Many(home, "phones")[0], "number"));
        Assert.Equal(new Value.Text("Mobile"), Cell(Many(home, "phones")[0], "phone_type_name"));
        var missing = Mapper.Map(NoAddress());
        Assert.Empty(Many(missing, "addresses"));
        Assert.Single(Many(missing, "phones"));
        Assert.Equal(new Value.Text("555"), Cell(Many(missing, "phones")[0], "number"));
    }

    [Fact]
    public void ReverseMapsTheFirstChildOfEachGroup()
    {
        AssertRow(AdaHome(), Mapper.ReverseMap(Mapper.Map(AdaHome())));
        AssertRow(NoAddress(), Mapper.ReverseMap(Mapper.Map(NoAddress())));
    }

    [Fact]
    public void CollapsesDuplicateJoinRowsIntoOneContactWithUniqueChildren()
    {
        var rows = Mapper.Collapse([AdaHome(), AdaWork(), AdaHome(), NoAddress()]);
        Assert.Equal(2, rows.Count);
        var ada = rows[0];
        Assert.Equal(new Value.Int(7), Cell(ada, "id"));
        Assert.Equal(2, Many(ada, "addresses").Count);
        Assert.Equal(new Value.Int(101), Cell(Many(ada, "addresses")[0], "id"));
        Assert.Equal(new Value.Int(102), Cell(Many(ada, "addresses")[1], "id"));
        Assert.Equal(new Value.Text("1 Parliament Sq"), Cell(Many(ada, "addresses")[1], "line1"));
        Assert.Single(Many(ada, "phones"));
        Assert.Equal(new Value.Int(8), Cell(rows[1], "id"));
        Assert.Empty(Many(rows[1], "addresses"));
        Assert.Single(Many(rows[1], "phones"));
    }

    [Fact]
    public void CollapsesAlreadyMappedRows()
    {
        var rows = Mapper.Collapse([Mapper.Map(AdaHome()), Mapper.Map(AdaWork())]);
        Assert.Equal(2, Many(rows[0], "addresses").Count);
        Assert.Single(Many(rows[0], "phones"));
    }

    [Fact]
    public void SkipsRowsWithoutAParentKeyAndChildrenWithoutAGroupKey()
    {
        var orphan = new Row(AdaHome()) { ["id"] = Value.Null.Instance };
        var mapped = Mapper.Map(AdaHome());
        mapped["addresses"] = new MappedValue.Many(
        [
            new()
            {
                ["line1"] = new MappedValue.Cell("x"),
                ["city"] = new MappedValue.Cell("y"),
            },
            new()
            {
                ["id"] = new MappedValue.Cell(101),
                ["line1"] = new MappedValue.Cell("dup"),
                ["city"] = new MappedValue.Cell("London"),
            },
        ]);
        var extra = Mapper.Map(AdaWork());
        extra["addresses"] = new MappedValue.Many(
        [
            new()
            {
                ["line1"] = new MappedValue.Cell("ghost"),
                ["city"] = new MappedValue.Cell("None"),
            },
        ]);
        var ada = Mapper.Collapse([Mapper.Map(orphan), mapped, extra])[0];
        Assert.Equal(
            ["x", "dup"],
            Many(ada, "addresses").Select(address => ((MappedValue.Cell)address["line1"]).Value));
    }

    [Fact]
    public void FallsBackToTheSpecKeyWhenItIsNotAMappedDestField()
    {
        var names = new FlatToTypeMapper(new TypeMap
        {
            Key = "id",
            Fields = new Dictionary<string, TypeMapField> { ["name"] = "name" },
        });
        var rows = names.Collapse([new Row([new("id", 1), new("name", "ada")])]);
        Assert.Equal(new Value.Int(1), Cell(rows[0], "id"));
        Assert.Equal(new Value.Text("ada"), Cell(rows[0], "name"));
    }

    [Fact]
    public void OmitsNestedGroupFieldsWhenCompilingACollection()
    {
        var nested = new FlatToTypeMapper(new TypeMap
        {
            Key = "id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "id",
                ["items"] = new TypeMapGroup
                {
                    Key = "item_id",
                    Fields = new Dictionary<string, TypeMapField>
                    {
                        ["id"] = "item_id",
                        ["title"] = "item_title",
                        ["kids"] = new TypeMapGroup
                        {
                            Key = "kid_id",
                            Fields = new Dictionary<string, TypeMapField> { ["n"] = "kid_n" },
                        },
                    },
                },
            },
        });
        var mapped = nested.Map(new Row([new("id", 1), new("item_id", 2)]));
        Assert.Equal(new Value.Int(1), Cell(mapped, "id"));
        Assert.Single(Many(mapped, "items"));
        Assert.Equal(new Value.Int(2), Cell(Many(mapped, "items")[0], "id"));
        Assert.Equal(Value.Null.Instance, Cell(Many(mapped, "items")[0], "title"));
        Assert.False(Many(mapped, "items")[0].ContainsKey("kids"));
        Assert.Equal(Value.Null.Instance, Cell(nested.Map(new Row()), "id"));
        var reversed = nested.ReverseMap([]);
        Assert.Equal(Value.Null.Instance, reversed["id"]);
        Assert.Equal(Value.Null.Instance, reversed["item_id"]);
        Assert.Equal(Value.Null.Instance, reversed["item_title"]);
    }
}
