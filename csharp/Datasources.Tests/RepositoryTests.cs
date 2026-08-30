using Datasources;
using Datasources.Builders;

namespace Datasources.Tests;

public class RepositoryTests
{
    public RepositoryTests() => TypeConverter.Reset();

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

    sealed class Row
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
        bool assignIdentityOnAdd = true,
        IReadOnlyDictionary<string, IFieldConverter>? converters = null)
        where T : new()
    {
        TypeConverter.Use("memory", new ProviderTypes { Rename = false });
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new Repository<T>(new RepositoryBuilderOptions<T>
        {
            Connection = connection,
            Ids = id ?? ["id"],
            AssignIdentityOnAdd = assignIdentityOnAdd,
            Mapping = TypeConverter.Mapping<T>("memory", converters: converters, name: "users")
        }), command);
    }

    static (Repository<Row> Repo, MockCommand Command) Create(
        IReadOnlyList<string>? id = null,
        bool assignIdentityOnAdd = true,
        IReadOnlyDictionary<string, IFieldConverter>? converters = null) =>
        Create<Row>(id, assignIdentityOnAdd, converters);

    [Fact]
    public async Task GetBuildsSelectAndMapsRows()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader(new object?[]
        {
            new Dictionary<string, Value> { ["id"] = 1, ["name"] = "a" }
        });
        var rows = new List<Row>();
        await foreach (var row in repo.Get())
            rows.Add(row);
        Assert.Single(rows);
        Assert.Equal(1, rows[0].id);
        Assert.Equal("SELECT id, name FROM users", command.ExecuteReaderCalls[0].Text);
        Assert.Same(command.Connection, repo.Connection);
        command.Reader = new MockReader(new object?[]
        {
            new Dictionary<string, Value> { ["id"] = 1, ["name"] = "a" }
        });
        Assert.Equal(1, (await repo.GetSingle())!.id);
    }

    [Fact]
    public async Task GetUsesCallerQuery()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader([]);
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
        command.Reader = new MockReader([]);
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
        var obj = new Row { id = 1, name = "a" };
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
        var twice = new FieldConverter(v => v, v => v is int n ? n * 2 : v is long l ? l * 2 : v);
        var (repo, command) = Create<Composite>(
            id: ["org_id", "user_id"],
            assignIdentityOnAdd: true,
            converters: new Dictionary<string, IFieldConverter> { ["name"] = twice });
        await repo.Add(new Composite { org_id = 1, user_id = 2, name = 3 });
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal(new Value.Int(6), command.ExecuteCalls[0].Parameters![0].Value);
        await repo.Update(new Composite { org_id = 1, user_id = 2, name = 4 });
        Assert.Contains("org_id = $org_id AND user_id = $user_id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task MissingIdAndBadRowAndCtor()
    {
        Assert.Throws<DatasourceException>(() =>
            new Repository<Dictionary<string, Value>>(new RepositoryBuilderOptions<Dictionary<string, Value>>
            {
                Connection = new MockConnection()
            }));
        var (repo, command) = Create();
        command.Reader = new MockReader(new object?[] { "nope" });
        await Assert.ThrowsAsync<DatasourceException>(async () =>
        {
            await foreach (var _ in repo.Get()) { }
        });
        var (cells, _) = Create<Dictionary<string, Value>>();
        await Assert.ThrowsAsync<DatasourceException>(() =>
            cells.Update(new Dictionary<string, Value> { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            cells.Delete(new Dictionary<string, Value> { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            Create<IdOnly>().Repo.Update(new IdOnly { id = 1 }));
        TypeConverter.Use("memory", FieldConverters.Sql());
        var mapping = TypeConverter.Mapping<Item>("memory", name: "users");
        var connection2 = new MockConnection { Provider = "memory" };
        var command2 = new MockCommand(connection2);
        connection2.Command = command2;
        var typed = new Repository<Item>(new RepositoryBuilderOptions<Item>
        {
            Connection = connection2,
            Mapping = mapping
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
        TypeConverter.Use("mock", new ProviderTypes { Rename = false });
        var connection = new MockConnection();
        var fromId = CreateRepositoryBuilderOptions.FromType<Item>(typeof(Item), connection);
        Assert.Equal(new[] { "Id" }, fromId.Ids);
        Assert.Same(connection, fromId.Connection);
        Assert.Empty(CreateRepositoryBuilderOptions.FromType<Label>(connection).Ids);
        TypeConverter.Use("mock", new ProviderTypes { Rename = false });
        Assert.Same(connection, new Repository<Item>(fromId).Connection);
        TypeConverter.Use("mock", new ProviderTypes { Rename = false });
        Assert.Same(connection, new Repository<Item>(connection).Connection);
    }
}
