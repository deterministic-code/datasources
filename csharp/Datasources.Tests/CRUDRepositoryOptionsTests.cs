using Datasources;

namespace Datasources.Tests;

public class CRUDRepositoryOptionsTests
{
    sealed class UserRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    static IMapper<string, string> Entity() => new TransformMapper<string, string>(
        static name => name == "users" ? "people" : name,
        static name => name);

    static IMapper<string, string> Columns() => new TransformMapper<string, string>(
        static name => name switch
        {
            "id" => "user_id",
            "name" => "full_name",
            _ => name
        },
        static name => name);

    static (CRUDRepository<UserRow> Repo, MockCommand Command) Create(CRUDRepositoryOptions<UserRow> options)
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new CRUDRepository<UserRow>(connection, options), command);
    }

    [Fact]
    public async Task BuilderMapsEntityAndColumns()
    {
        var (repo, command) = Create(new CRUDRepositoryOptions<UserRow>
        {
            Entity = "users",
            EntityMapper = Entity(),
            PropertyMappings = Columns()
        });
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle();
        Assert.Equal("SELECT full_name, user_id FROM people", command.ExecuteReaderCalls[0].Text);
        command.Response = new Response(Value.Null.Instance, 1);
        var created = await repo.Create(new UserRow { id = 1, name = "a" });
        Assert.Equal(1, created.id);
        Assert.Equal("INSERT INTO people (full_name) VALUES ($full_name)", command.ExecuteCalls[0].Text);
        var updated = await repo.Update(new UserRow { id = 1, name = "a" });
        Assert.Equal(1, updated.RowsAffected);
        Assert.StartsWith("UPDATE people SET", command.ExecuteCalls[1].Text);
        var removed = await repo.Delete(new UserRow { id = 1, name = "a" });
        Assert.Equal(1, removed.RowsAffected);
        Assert.StartsWith("DELETE FROM people", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public void StatementsUseTheMappedTableAndColumns()
    {
        var statements = new CRUDRepositoryStatements("users", ["id", "name"]);
        Assert.Equal("SELECT id, name FROM users", statements.Select().GetText());
        Assert.Equal(
            "INSERT INTO users (id) VALUES ($id)",
            statements.Insert([new Parameter("id", 1)]).GetText());
        var updated = statements.Update([new Parameter("name", "a")]);
        Assert.Equal("UPDATE users SET name = $name", updated.GetText());
        Assert.Equal("name", Assert.Single(UpdateStatement.ParameterList([new Parameter("name", "a")])).Name);
        Assert.Equal("DELETE FROM users", statements.Delete().GetText());
        var inserted = statements.SelectInsertedId("id");
        Assert.Equal("users", inserted.Table);
        Assert.Equal("id", inserted.Column);
        var error = Assert.Throws<InvalidOperationException>(inserted.GetText);
        Assert.Equal("provider must implement SelectInsertIdStatement", error.Message);
    }
}
