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
            ColumnMappings = Columns()
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
    public void PerStatementMappersRewriteFinishedSql()
    {
        var statements = new RepositoryStatements(
            "users",
            ["id", "name"],
            new TransformMapper<ISelectStatement, ISelectStatement>(
                query => new SelectStatement($"{query.GetText()} LIMIT 1", query.Where),
                query => query),
            new TransformMapper<IInsertStatement, IInsertStatement>(
                insert => new InsertStatement($"{insert.GetText()} RETURNING id"),
                insert => insert),
            new TransformMapper<IUpdateStatement, IUpdateStatement>(
                update => new UpdateStatement($"{update.GetText()} RETURNING id", update.Where),
                update => update),
            new TransformMapper<IDeleteStatement, IDeleteStatement>(
                delete => DeleteStatement.Raw($"{delete.GetText()} RETURNING id", delete.Where),
                delete => delete));
        Assert.Equal("SELECT id, name FROM users LIMIT 1", statements.Query().GetText());
        Assert.Equal(
            "INSERT INTO users (id) VALUES ($id) RETURNING id",
            statements.Insert([new Parameter("id", 1)]).GetText());
        var updated = statements.Update([new Parameter("name", "a")]);
        Assert.Equal("UPDATE users SET name = $name RETURNING id", updated.GetText());
        Assert.Equal("name", Assert.Single(UpdateStatement.ParameterList([new Parameter("name", "a")])).Name);
        Assert.Equal("DELETE FROM users RETURNING id", statements.Delete().GetText());
        Assert.Equal("users.id", statements.SelectInsertedId("id").GetText());
    }

    [Fact]
    public void ReturnMapperRewritesFinishedSql()
    {
        var options = new CRUDRepositoryOptions<UserRow>
        {
            Return = new TransformMapper<IReturnStatement, IReturnStatement>(
                ret => new ReturnStatement($"{ret.GetText()} /* mapped */"),
                ret => ret)
        };
        var built = new ReturnStatement("add", [new Parameter("id", 1)]);
        var statement = options.Return is { } map ? map.Map(built) : built;
        Assert.Equal("SELECT add($id) /* mapped */", statement.GetText());
    }

    [Fact]
    public async Task ReturnMapperReadsTheInsertedId()
    {
        var (repo, command) = Create(new CRUDRepositoryOptions<UserRow>
        {
            Entity = "users",
            Return = new TransformMapper<IReturnStatement, IReturnStatement>(
                ret => new ReturnStatement($"{ret.GetText()} /* mapped */"),
                ret => ret)
        });
        command.Reader = new MockReader<Row>([new Row { ["add"] = 2 }]);
        command.Response = new Response(Value.Null.Instance, 1);
        var created = await repo.Create(new UserRow { name = "a" });
        Assert.Equal(2, created.id);
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal("SELECT id($name) /* mapped */", command.ExecuteReaderCalls[0].Text);
    }

    [Fact]
    public async Task BuilderThenStatementMappers()
    {
        var (repo, command) = Create(new CRUDRepositoryOptions<UserRow>
        {
            Entity = "users",
            EntityMapper = Entity(),
            ColumnMappings = Columns(),
            Query = new TransformMapper<ISelectStatement, ISelectStatement>(
                query => new SelectStatement($"{query.GetText()} LIMIT 1", query.Where),
                query => query),
            Insert = new TransformMapper<IInsertStatement, IInsertStatement>(
                insert => new InsertStatement($"{insert.GetText()} RETURNING id"),
                insert => insert),
            Update = new TransformMapper<IUpdateStatement, IUpdateStatement>(
                update => new UpdateStatement($"{update.GetText()} RETURNING id", update.Where),
                update => update),
            Delete = new TransformMapper<IDeleteStatement, IDeleteStatement>(
                delete => DeleteStatement.Raw($"{delete.GetText()} RETURNING id", delete.Where),
                delete => delete)
        });
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle();
        Assert.Equal("SELECT full_name, user_id FROM people LIMIT 1", command.ExecuteReaderCalls[0].Text);
        command.Response = new Response(Value.Null.Instance, 1);
        var row = new UserRow { id = 1, name = "a" };
        var created = await repo.Create(row);
        Assert.Equal(1, created.id);
        Assert.Equal("INSERT INTO people (full_name) VALUES ($full_name) RETURNING id", command.ExecuteCalls[0].Text);
        var updated = await repo.Update(row);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal("UPDATE people SET full_name = $full_name WHERE user_id = $user_id RETURNING id", command.ExecuteCalls[1].Text);
        var removed = await repo.Delete(row);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Equal("DELETE FROM people WHERE user_id = $user_id RETURNING id", command.ExecuteCalls[2].Text);
    }
}
