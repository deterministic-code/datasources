using Datasources;

namespace Datasources.Tests;

public class StatementMappingsTests
{
    static readonly StatementMappings Mappings = new(
        new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["User"] = "people",
            ["Role"] = "roles",
        }),
        new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Name"] = "full_name",
            ["title"] = "book.title",
            ["id"] = "user_id",
        }));

    static Value Cell(string name, Value value) =>
        value is Value.Text text ? $"{name}:{text.Item.ToUpperInvariant()}" : value;

    [Fact]
    public void MapsSelect()
    {
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
        var mapped = Assert.IsType<SelectStatement>(Mappings.Apply(select, Cell));
        Assert.Equal("people", mapped.Table);
        Assert.Equal(["full_name", "book.title", "book.title", "alias.other", "\"schema.name\".user_id"], mapped.Fields);
        Assert.True(mapped.Distinct);
        Assert.Equal(5, mapped.Limit);
        Assert.Equal(1, mapped.Offset);
        Assert.Equal("full_name", Assert.Single(mapped.GroupBy));
        Assert.Equal("full_name", Assert.Single(mapped.OrderBy).Column);
        Assert.Equal(SortDirection.Desc, Assert.Single(mapped.OrderBy).Direction);
        Assert.Equal("people.user_id", ((Operator.Equals)((Join)mapped.Joins[0]).On!).Left is Identifier left ? left.Name : "");
        Assert.Equal("logs", mapped.Joins[1].Table);
        Assert.Null(mapped.Joins[1].On);
        Assert.Contains("full_name = $full_name", mapped.GetText());
        Assert.Equal(new Value.Text("Name:BEA"), Assert.Single(mapped.Where!.Parameters).Value);
        Assert.Equal("Name", Mappings.Property("full_name"));
        Assert.Equal("Name", Mappings.Property("people.full_name"));
        Assert.Equal("other", Mappings.Property("other"));
        Assert.Equal("people.missing", Mappings.Property("people.missing"));
    }

    [Fact]
    public void MapsWrites()
    {
        var insert = Assert.IsType<InsertStatement>(Mappings.Apply(
            new InsertStatement("User", [new Parameter("Name", "Ada")]), Cell));
        Assert.Equal("INSERT INTO people (full_name) VALUES ($full_name)", insert.GetText());
        Assert.Equal(new Value.Text("Ada"), Assert.Single(insert.Fields).Value);

        var update = Assert.IsType<UpdateStatement>(Mappings.Apply(
            new UpdateStatement("User", [new Parameter("Name", "Ada")], new Where(new Operator.Equals("Name", "Bea"))),
            Cell));
        Assert.Equal(
            "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where",
            update.GetText());
        Assert.Equal(new Value.Text("Ada"), update.Parameters[0].Value);
        Assert.Equal("full_name__where", update.Parameters[1].Name);
        Assert.Equal(new Value.Text("Name:BEA"), update.Parameters[1].Value);

        var delete = Assert.IsType<DeleteStatement>(Mappings.Apply(
            new DeleteStatement("User", new Where(new Operator.Equals("Name", "Bea"))), Cell));
        Assert.Equal("DELETE FROM people WHERE full_name = $full_name", delete.GetText());

        var inserted = Assert.IsType<SelectInsertedIdStatement>(Mappings.Apply(
            new SelectInsertedIdStatement("User", "id"), Cell));
        Assert.Equal("people", inserted.Table);
        Assert.Equal("user_id", inserted.Column);
    }

    [Fact]
    public void LeavesRawAndOtherStatements()
    {
        var rawInsert = new InsertStatement("INSERT INTO t VALUES (1)");
        Assert.Same(rawInsert, Mappings.Apply(rawInsert, Cell));
        var rawUpdate = new UpdateStatement("x");
        Assert.Same(rawUpdate, Mappings.Apply(rawUpdate, Cell));
        var rawDelete = DeleteStatement.Raw("DELETE FROM t");
        Assert.Same(rawDelete, Mappings.Apply(rawDelete, Cell));
        var bare = new SelectStatement("SELECT 1");
        Assert.Same(bare, Mappings.Apply(bare, Cell));
        var call = new ExecuteStoredProcedureStatement("CALL save_user()");
        Assert.Same(call, Mappings.Apply(call, Cell));
        Assert.Null(Mappings.Apply((IStatement?)null, Cell));
        Assert.Null(Mappings.Apply((IWhere?)null, Cell));
        var text = new WhereText("name = $name");
        Assert.Same(text, Mappings.Apply(text, Cell));
        var kept = Assert.IsType<DeleteStatement>(Mappings.Apply(new DeleteStatement("User", text), Cell));
        Assert.Same(text, kept.Where);
        Assert.Equal("DELETE FROM people WHERE name = $name", kept.GetText());
    }
}
