using Datasources;
using Datasources.Builders;

namespace Datasources.Tests;

public class StatementMapperTests
{
    sealed class UserRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    static IMapper<string, string> Names() => new TransformMapper<string, string>(
        static name => name switch
        {
            "users" => "people",
            "id" => "user_id",
            "name" => "full_name",
            _ => name
        },
        static name => name);

    static (Repository<UserRow> Repo, MockCommand Command) Create(StatementMapper<UserRow> mapper)
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new Repository<UserRow>(new RepositoryBuilderOptions<UserRow>
        {
            Connection = connection,
            Entity = "users",
            AssignIdentityOnCreate = false,
            StatementMapper = mapper
        }), command);
    }

    [Fact]
    public async Task BuilderMapsEntityAndColumns()
    {
        var names = Names();
        var (repo, command) = Create(new StatementMapper<UserRow> { Entity = names, Columns = names });
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle();
        Assert.Equal("SELECT full_name, user_id FROM people", command.ExecuteReaderCalls[0].Text);
        await repo.Create(new UserRow { id = 1, name = "a" });
        Assert.Equal("INSERT INTO people (id, name) VALUES ($id, $name)", command.ExecuteCalls[0].Text);
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
                query => new Query($"{query.GetText()} LIMIT 1", query.Where),
                query => query),
            new TransformMapper<IInsertStatement, IInsertStatement>(
                insert => new InsertStatement($"{insert.GetText()} RETURNING id"),
                insert => insert),
            new TransformMapper<IUpdateStatement, IUpdateStatement>(
                update => new UpdateStatement($"{update.GetText()} RETURNING id", update.Where),
                update => update),
            new TransformMapper<IDeleteStatement, IDeleteStatement>(
                delete => new DeleteStatement($"{delete.GetText()} RETURNING id", delete.Where),
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
    public void StoredProcedureMapperRewritesFinishedSql()
    {
        var mapper = new StatementMapper<UserRow>
        {
            StoredProcedure = new TransformMapper<IStoredProcedureStatement, IStoredProcedureStatement>(
                proc => new StoredProcedureStatement($"{proc.GetText()} /* mapped */"),
                proc => proc)
        };
        var built = StatementBuilder.CreateStoredProcedure("save", [new Parameter("id", 1)]);
        var statement = mapper.StoredProcedure is { } map ? map.Map(built) : built;
        Assert.Equal("CALL save($id) /* mapped */", statement.GetText());
    }

    [Fact]
    public async Task BuilderThenStatementMapper()
    {
        var names = Names();
        var (repo, command) = Create(new StatementMapper<UserRow>
        {
            Entity = names,
            Columns = names,
            Query = new TransformMapper<IQuery, IQuery>(
                query => new Query($"{query.GetText()} LIMIT 1", query.Where),
                query => query),
            Insert = new TransformMapper<IInsertStatement, IInsertStatement>(
                insert => new InsertStatement($"{insert.GetText()} RETURNING id"),
                insert => insert),
            Update = new TransformMapper<IUpdateStatement, IUpdateStatement>(
                update => new UpdateStatement($"{update.GetText()} RETURNING id", update.Where),
                update => update),
            Delete = new TransformMapper<IDeleteStatement, IDeleteStatement>(
                delete => new DeleteStatement($"{delete.GetText()} RETURNING id", delete.Where),
                delete => delete)
        });
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle();
        Assert.Equal("SELECT full_name, user_id FROM people LIMIT 1", command.ExecuteReaderCalls[0].Text);
        var row = new UserRow { id = 1, name = "a" };
        await repo.Create(row);
        Assert.Equal("INSERT INTO people (id, name) VALUES ($id, $name) RETURNING id", command.ExecuteCalls[0].Text);
        await repo.Update(row);
        Assert.Equal("UPDATE people SET name = $name WHERE id = $id RETURNING id", command.ExecuteCalls[1].Text);
        await repo.Delete(row);
        Assert.Equal("DELETE FROM people WHERE id = $id RETURNING id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public void RowsDelegateOnStatementMapper()
    {
        var mapper = new StatementMapper<UserRow>
        {
            Rows = new TransformMapper<UserRow, UserRow>(
                row => { row.name = row.name.ToUpperInvariant(); return row; },
                row => row)
        };
        IMapper<UserRow, UserRow> rows = mapper;
        Assert.Equal("A", rows.Map(new UserRow { name = "a" }).name);
    }
}
