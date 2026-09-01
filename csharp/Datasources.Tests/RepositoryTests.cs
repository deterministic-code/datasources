using Datasources;
using Datasources.Builders;

namespace Datasources.Tests;

public class RepositoryTests
{
    sealed class Item
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Label
    {
        public string Name { get; set; } = "";
    }

    sealed class IdOnly
    {
        public int id { get; set; }
    }

    sealed class UserRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class Composite
    {
        public int org_id { get; set; }
        public int user_id { get; set; }
        public int name { get; set; }
    }

    static (Repository<T> Repo, MockCommand Command) Create<T>(
        IReadOnlyList<string>? id = null,
        bool assignIdentityOnAdd = true)
        where T : new()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new Repository<T>(new RepositoryBuilderOptions<T>
        {
            Connection = connection,
            Ids = id ?? ["id"],
            AssignIdentityOnAdd = assignIdentityOnAdd,
            Table = "users"
        }), command);
    }

    static (Repository<UserRow> Repo, MockCommand Command) Create(
        IReadOnlyList<string>? id = null,
        bool assignIdentityOnAdd = true) =>
        Create<UserRow>(id, assignIdentityOnAdd);

    [Fact]
    public async Task GetBuildsSelectAndMapsRows()
    {
        var (repo, command) = Create();
        repo.Use(new PassMiddleware());
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        var rows = new List<UserRow>();
        await foreach (var row in repo.Get())
            rows.Add(row);
        Assert.Single(rows);
        Assert.Equal(1, rows[0].id);
        Assert.Equal("SELECT id, name FROM users", command.ExecuteReaderCalls[0].Text);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Assert.Equal(1, (await repo.GetSingle())!.id);
    }

    [Fact]
    public async Task GetUsesCallerQuery()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        var where = WhereBuilder.Equals("id", 9);
        await repo.GetSingle(StatementBuilder.CreateQuery("users", ["id"], where));
        Assert.Equal("SELECT id FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        Assert.Null(await repo.GetSingle());
    }

    [Fact]
    public async Task GetAttachesWhereToDefaultSelect()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        await repo.GetSingle(WhereBuilder.Equals("id", 9));
        Assert.Equal("SELECT id, name FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        await foreach (var _ in repo.Get(WhereBuilder.Equals("id", 9)))
            break;
    }

    [Fact]
    public async Task AddUpdateDeleteUseBuilders()
    {
        var (repo, command) = Create(assignIdentityOnAdd: false);
        var obj = new UserRow { id = 1, name = "a" };
        await repo.Add(obj);
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", command.ExecuteCalls[0].Text);
        await repo.Update(obj);
        Assert.Equal("UPDATE users SET name = $name WHERE id = $id", command.ExecuteCalls[1].Text);
        Assert.Equal(new Value.Text("a"), command.ExecuteCalls[1].Parameters![0].Value);
        Assert.Equal(new Value.Int(1), command.ExecuteCalls[1].Parameters![1].Value);
        await repo.Delete(obj);
        Assert.Equal("DELETE FROM users WHERE id = $id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public async Task CompositeIdAndAssignIdentityOnAddAndOverrides()
    {
        var (repo, command) = Create<Composite>(id: ["org_id", "user_id"], assignIdentityOnAdd: true);
        await repo.Add(new Composite { org_id = 1, user_id = 2, name = 3 });
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal(new Value.Int(3), command.ExecuteCalls[0].Parameters![0].Value);
        await repo.Update(new Composite { org_id = 1, user_id = 2, name = 4 });
        Assert.Contains("org_id = $org_id AND user_id = $user_id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task MissingIdAndBadRowAndCtor()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([null]);
        await Assert.ThrowsAsync<DatasourceException>(async () =>
        {
            await foreach (var _ in repo.Get()) { }
        });
        var (cells, _) = Create<Dictionary<string, Value>>();
        await Assert.ThrowsAsync<DatasourceException>(() =>
            cells.Update(new Row { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            cells.Delete(new Row { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            Create<IdOnly>().Repo.Update(new IdOnly { id = 1 }));
        var connection2 = new MockConnection { Provider = "memory" };
        var command2 = new MockCommand(connection2);
        connection2.Command = command2;
        var typed = new Repository<Item>(new RepositoryBuilderOptions<Item>
        {
            Connection = connection2,
            Table = "users"
        });
        await typed.Add(new Item { Id = 1, Name = "a" });
        Assert.Contains("INSERT INTO users", command2.ExecuteCalls[0].Text);
        var where = WhereBuilder.Equals("name", "z");
        await typed.Delete(new Item { Name = "z" }, where);
        Assert.Equal("DELETE FROM users WHERE name = $name", command2.ExecuteCalls[1].Text);
    }

    [Fact]
    public void FromTypeBuildsOptionsFromConnection()
    {
        var connection = new MockConnection();
        var fromId = CreateRepositoryBuilderOptions.FromType<Item>(typeof(Item), connection);
        Assert.Equal(new[] { "Id" }, fromId.Ids);
        Assert.Same(connection, fromId.Connection);
        Assert.Empty(CreateRepositoryBuilderOptions.FromType<Label>(connection).Ids);
        Assert.NotNull(new Repository<Item>(fromId));
        Assert.NotNull(new Repository<Item>(connection));
        Assert.Same(connection, RepositoryBuilder<Item>.Parts(fromId).Connection);
    }

    sealed class PassMiddleware : ICommandMiddleware
    {
        public Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(
            string query, IReadOnlyList<Parameter>? parameters) =>
            Task.FromResult<(string Query, IReadOnlyList<Parameter>? Parameters)?>((query, parameters));

        public Task AfterQuery(
            string query, IReadOnlyList<Parameter>? parameters, IReadOnlyList<Row> results,
            double elapsedMs, Exception? error = null) =>
            Task.CompletedTask;
    }
}
