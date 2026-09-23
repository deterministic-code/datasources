using Datasources;

namespace Datasources.Tests;

public class CRUDMapperOptionsTests
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

    static (CRUDRepository<UserRow> Repo, MockCommand Command) Create(CRUDMapperOptions<UserRow> mapper)
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new CRUDRepository<UserRow>(connection, new RepositoryOptions<UserRow>
        {
            Entity = "users",
            CRUDMapper = mapper
        }), command);
    }

    [Fact]
    public async Task BuilderMapsEntityAndColumns()
    {
        var (repo, command) = Create(new CRUDMapperOptions<UserRow> { Entity = Entity(), Columns = Columns() });
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle();
        Assert.Equal("SELECT full_name, user_id FROM people", command.ExecuteReaderCalls[0].Text);
        await repo.Create(new UserRow { id = 1, name = "a" });
        Assert.Equal("INSERT INTO people (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        await repo.Update(new UserRow { id = 1, name = "a" });
        Assert.StartsWith("UPDATE people SET", command.ExecuteCalls[1].Text);
        await repo.Delete(new UserRow { id = 1, name = "a" });
        Assert.StartsWith("DELETE FROM people", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public void PerStatementMappersRewriteFinishedSql()
    {
        var statements = new RepositoryStatements(
            "users",
            ["id", "name"],
            new TransformMapper<IQuery, IQuery>(
                query => new QueryStatement($"{query.GetText()} LIMIT 1", query.Where),
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
        Assert.Equal(
            "UPDATE users SET name = $name RETURNING id",
            statements.Update([new Parameter("name", "a")]).GetText());
        Assert.Equal("DELETE FROM users RETURNING id", statements.Delete().GetText());
    }

    [Fact]
    public void ReturnMapperRewritesFinishedSql()
    {
        var mapper = new CRUDMapperOptions<UserRow>
        {
            Return = new TransformMapper<IReturnStatement, IReturnStatement>(
                ret => new ReturnStatement($"{ret.GetText()} /* mapped */"),
                ret => ret)
        };
        var built = new ReturnStatement("add", [new Parameter("id", 1)]);
        var statement = mapper.Return is { } map ? map.Map(built) : built;
        Assert.Equal("SELECT add($id) /* mapped */", statement.GetText());
    }

    [Fact]
    public async Task BuilderThenCRUDMapper()
    {
        var (repo, command) = Create(new CRUDMapperOptions<UserRow>
        {
            Entity = Entity(),
            Columns = Columns(),
            Query = new TransformMapper<IQuery, IQuery>(
                query => new QueryStatement($"{query.GetText()} LIMIT 1", query.Where),
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
        var row = new UserRow { id = 1, name = "a" };
        await repo.Create(row);
        Assert.Equal("INSERT INTO people (name) VALUES ($name) RETURNING id", command.ExecuteCalls[0].Text);
        await repo.Update(row);
        Assert.Equal("UPDATE people SET name = $name WHERE id = $id RETURNING id", command.ExecuteCalls[1].Text);
        await repo.Delete(row);
        Assert.Equal("DELETE FROM people WHERE id = $id RETURNING id", command.ExecuteCalls[2].Text);
    }
}
