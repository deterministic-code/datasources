using Datasources;

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

    sealed class Stamp
    {
        public int id { get; set; }
        public string name { get; set; } = "";
        public DateTime? created { get; set; }
        public byte[]? version { get; set; }
    }

    sealed class AddressView
    {
        public int Id { get; set; }
        public string Line1 { get; set; } = "";
        public string Description { get; set; } = "";
    }

    sealed class ContactRow
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
        public string GivenName { get; set; } = "";
        public bool IsStarred { get; set; }
        public string Uuid { get; set; } = "new-id";
        public int GenderTypeId { get; set; }
    }

    static (CRUDRepository<T> Repo, MockCommand Command) Create<T>(
        IReadOnlyList<string>? id = null)
        where T : new()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new CRUDRepository<T>(connection, new CRUDRepositoryOptions<T>
        {
            Ids = id ?? ["id"],
            Entity = "users"
        }), command);
    }

    static (CRUDRepository<UserRow> Repo, MockCommand Command) Create(
        IReadOnlyList<string>? id = null) =>
        Create<UserRow>(id);

    [Fact]
    public async Task GetBuildsSelectAndMapsRows()
    {
        var (repo, command) = Create();
        repo.AddMiddleware(new PassMiddleware());
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        var rows = new List<UserRow>();
        await foreach (var row in repo.Read())
            rows.Add(row);
        Assert.Single(rows);
        Assert.Equal(1, rows[0].id);
        Assert.Equal("SELECT id, name FROM users", command.ExecuteReaderCalls[0].Text);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Assert.Equal(1, (await repo.ReadSingle())!.id);
    }

    [Fact]
    public async Task GetUsesCallerQuery()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        var where = new Where(new Operator.Equals("id", 9));
        await repo.ReadSingle(new SelectStatement(new() { Table = "users", Fields = ["id"], Where = where }));
        Assert.Equal("SELECT id FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        Assert.Null(await repo.ReadSingle());
    }

    [Fact]
    public async Task GetAttachesWhereToDefaultSelect()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle(new Where(new Operator.Equals("id", 9)));
        Assert.Equal("SELECT id, name FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        await foreach (var _ in repo.Read(new Where(new Operator.Equals("id", 9))))
            break;
    }

    [Fact]
    public async Task ReadCallsStoredProcedure()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Parameter[] parameters = [new("id", 9)];
        var call = new ExecuteStoredProcedureStatement("get_user", parameters);
        var rows = new List<UserRow>();
        await foreach (var row in repo.Read(call, parameters))
            rows.Add(row);
        Assert.Equal(1, rows[0].id);
        Assert.Equal("a", rows[0].name);
        Assert.Equal("CALL get_user($id)", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Assert.Equal(1, (await repo.ReadSingle(call, parameters))!.id);
    }

    [Fact]
    public async Task AddUpdateDeleteUseBuilders()
    {
        var (repo, command) = Create();
        command.Response = new Response(new Value.Int(9), 1);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 9 }]);
        var obj = new UserRow { id = 0, name = "a" };
        var created = await repo.Create(obj);
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal("users.id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(9, created.id);
        var updated = await repo.Update(obj);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal("UPDATE users SET name = $name WHERE id = $id", command.ExecuteCalls[1].Text);
        Assert.Equal(new Value.Text("a"), command.ExecuteCalls[1].Parameters![0].Value);
        Assert.Equal(new Value.Int(9), command.ExecuteCalls[1].Parameters![1].Value);
        var removed = await repo.Delete(obj);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Equal("DELETE FROM users WHERE id = $id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public async Task CreateIncludesAssignedId()
    {
        var (repo, command) = Create();
        command.Response = new Response(Value.Null.Instance, 1);
        var created = await repo.Create(new UserRow { id = 42, name = "ada" });
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", command.ExecuteCalls[0].Text);
        Assert.Empty(command.ExecuteReaderCalls);
        Assert.Equal(42, created.id);
    }

    [Fact]
    public async Task CreateMapperWritesConfiguredId()
    {
        var (repo, command) = Create<Item>(id: ["Id"]);
        command.Response = new Response(new Value.Int(4), 1);
        command.Reader = new MockReader<Row>([new Row { ["Id"] = 4 }]);
        var row = new Item { Name = "a" };
        var created = await repo.Create(row);
        Assert.Equal(4, created.Id);
        Assert.Contains("INSERT INTO users (Name)", command.ExecuteCalls[0].Text);
    }

    [Fact]
    public async Task CompositeIdIncludesAssignedIdsOnCreate()
    {
        var (repo, command) = Create<Composite>(id: ["org_id", "user_id"]);
        var created = await repo.Create(new Composite { org_id = 1, user_id = 2, name = 3 });
        Assert.Equal(1, created.org_id);
        Assert.Equal(2, created.user_id);
        Assert.Equal("INSERT INTO users (name, org_id, user_id) VALUES ($name, $org_id, $user_id)", command.ExecuteCalls[0].Text);
        Assert.Equal(new Value.Int(3), command.ExecuteCalls[0].Parameters![0].Value);
        command.Response = new Response(Value.Null.Instance, 1);
        var updated = await repo.Update(new Composite { org_id = 1, user_id = 2, name = 4 });
        Assert.Equal(1, updated.RowsAffected);
        Assert.Contains("org_id = $org_id AND user_id = $user_id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task CreateOmitsNullDatetimeAndVersionAndUpdateWritesThem()
    {
        var (repo, command) = Create<Stamp>();
        command.Response = new Response(new Value.Int(7), 1);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 7 }, new Row { ["id"] = 7 }]);
        var created = await repo.Create(new Stamp { name = "ada" });
        Assert.Equal(7, created.id);
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        var updated = await repo.Update(new Stamp { id = 1, name = "ada" });
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal("UPDATE users SET created = $created, name = $name, version = $version WHERE id = $id", command.ExecuteCalls[1].Text);
        var grace = await repo.Create(new Stamp
        {
            name = "grace",
            created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            version = [9, 9]
        });
        Assert.Equal(7, grace.id);
        Assert.Equal(
            "INSERT INTO users (created, name, version) VALUES ($created, $name, $version)",
            command.ExecuteCalls[2].Text);

        var (rows, rowCommand) = Create<Row>();
        rowCommand.Response = new Response(new Value.Int(7), 1);
        rowCommand.Reader = new MockReader<Row>([new Row { ["id"] = 7 }]);
        var createdRow = await rows.Create(new Row
        {
            ["id"] = 0,
            ["name"] = "ada",
            ["created"] = Value.Null.Instance,
            ["version"] = Value.Null.Instance
        });
        Assert.Equal(new Value.Int(7), createdRow["id"]);
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", rowCommand.ExecuteCalls[0].Text);
        var updatedRow = await rows.Update(new Row
        {
            ["id"] = 1,
            ["name"] = "ada-v2",
            ["created"] = Value.Null.Instance,
            ["version"] = Value.Null.Instance
        });
        Assert.Equal(1, updatedRow.RowsAffected);
        Assert.Equal("UPDATE users SET created = $created, name = $name, version = $version WHERE id = $id", rowCommand.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task CreateAndUpdateWriteReadPropertiesOnly()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<AddressView>(connection, new CRUDRepositoryOptions<AddressView>
        {
            Ids = ["Id"],
            Entity = "addresses",
            ReadProperties = ["Id", "Line1"]
        });
        command.Response = new Response(new Value.Int(1), 1);
        command.Reader = new MockReader<Row>([new Row { ["Id"] = 1 }]);
        await repo.Create(new AddressView { Line1 = "1 Main", Description = "Home" });
        Assert.Equal("INSERT INTO addresses (Line1) VALUES ($Line1)", command.ExecuteCalls[0].Text);
        await repo.Update(new AddressView { Id = 1, Line1 = "2 Main", Description = "Work" });
        Assert.Equal("UPDATE addresses SET Line1 = $Line1 WHERE Id = $Id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task CreateAndUpdateHonorWriteProperties()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<AddressView>(connection, new CRUDRepositoryOptions<AddressView>
        {
            Ids = ["Id"],
            Entity = "addresses",
            ReadProperties = ["Id", "Line1", "Description"],
            CreateProperties = ["Line1"],
            UpdateProperties = ["Line1"],
        });
        command.Response = new Response(new Value.Int(1), 1);
        command.Reader = new MockReader<Row>([new Row { ["Id"] = 1 }]);
        await repo.Create(new AddressView { Line1 = "1 Main", Description = "Home" });
        Assert.Equal("INSERT INTO addresses (Line1) VALUES ($Line1)", command.ExecuteCalls[0].Text);
        await repo.Update(new AddressView { Id = 1, Line1 = "2 Main", Description = "Work" });
        Assert.Equal("UPDATE addresses SET Line1 = $Line1 WHERE Id = $Id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task CreateFillsMissingStringAndBoolZeros()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<ContactRow>(connection, new CRUDRepositoryOptions<ContactRow>
        {
            Ids = ["Id"],
            Entity = "contacts",
            ReadProperties = ["Id", "Name", "GivenName", "IsStarred", "Uuid", "GenderTypeId"],
            CreateProperties = ["Name", "GivenName", "IsStarred", "Uuid"],
        });
        command.Response = new Response(new Value.Int(1), 1);
        command.Reader = new MockReader<Row>([new Row { ["Id"] = 1 }]);
        await repo.Create(new ContactRow { GivenName = "Ada" });
        Assert.Equal(
            "INSERT INTO contacts (GivenName, IsStarred, Name, Uuid) VALUES ($GivenName, $IsStarred, $Name, $Uuid)",
            command.ExecuteCalls[0].Text);
        await repo.Create(new ContactRow { GivenName = "Ada", IsStarred = true, Name = "Ada Lovelace" });
        Assert.Contains("IsStarred", command.ExecuteCalls[1].Text);
        Assert.Equal(new Value.Bool(true), command.ExecuteCalls[1].Parameters!.Single(field => field.Name == "IsStarred").Value);
    }

    [Fact]
    public async Task ReadSingleByIdUsesIdColumns()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        Assert.Null(await repo.ReadSingleById(9));
        Assert.Equal("SELECT id, name FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);

        var (composite, generated) = Create<Composite>(id: ["org_id", "user_id"]);
        generated.Reader = new MockReader<Row>([]);
        Assert.Null(await composite.ReadSingleById(new Row { ["org_id"] = 1, ["user_id"] = 2 }));
        Assert.Contains("org_id = $org_id AND user_id = $user_id", generated.ExecuteReaderCalls[0].Text);
        var scalar = await Assert.ThrowsAsync<DatasourceException>(() => composite.ReadSingleById(1));
        Assert.Equal("Composite id requires a row keyed by each Id column", scalar.Message);
        var missing = await Assert.ThrowsAsync<DatasourceException>(() =>
            composite.ReadSingleById(new Row { ["org_id"] = 1 }));
        Assert.Contains("Missing Id value", missing.Message);
    }

    [Fact]
    public async Task MissingIdAndBadRowAndCtor()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([null]);
        await Assert.ThrowsAsync<DatasourceException>(async () =>
        {
            await foreach (var _ in repo.Read()) { }
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
        var typed = new CRUDRepository<Item>(connection2, new CRUDRepositoryOptions<Item>
        {
            Entity = "users"
        });
        command2.Response = new Response(Value.Null.Instance, 1);
        var created = await typed.Create(new Item { Id = 1, Name = "a" });
        Assert.Equal(1, created.Id);
        Assert.Contains("INSERT INTO users", command2.ExecuteCalls[0].Text);
        var where = new Where(new Operator.Equals("name", "z"));
        var removed = await typed.Delete(new Item { Name = "z" }, where);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Equal("DELETE FROM users WHERE name = $name", command2.ExecuteCalls[1].Text);
    }

    [Fact]
    public void FromTypeBuildsOptions()
    {
        var connection = new MockConnection();
        var fromId = CreateCRUDRepositoryOptions.FromType<Item>(typeof(Item));
        Assert.Equal(new[] { "Id" }, fromId.Ids);
        Assert.Equal(nameof(Item), fromId.Entity);
        Assert.Empty(CreateCRUDRepositoryOptions.FromType<Label>().Ids);
        Assert.Equal(nameof(Item), CreateReadonlyRepositoryMappingOptions.FromType<Item>().Entity);
        Assert.NotNull(new CRUDRepository<Item>(connection, fromId));
        Assert.NotNull(new CRUDRepository<Item>(connection));
        Assert.NotNull(new ReadonlyRepository<Item>(connection, fromId));
        Assert.NotNull(new ReadonlyRepository<Item>(connection));
        Assert.Same(connection, new CRUDRepositoryBuilder<Item>().Parts(connection, fromId).Connection);
    }

    sealed class PassMiddleware : ICommandMiddleware
    {
        public Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(
            string text, IReadOnlyList<Parameter>? parameters) =>
            Task.FromResult<(string Text, IReadOnlyList<Parameter>? Parameters)?>((text, parameters));

        public Task AfterCommand(
            string text, IReadOnlyList<Parameter>? parameters, IReadOnlyList<Row> results,
            double elapsedMs, Exception? error = null) =>
            Task.CompletedTask;
    }
}
