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
        var where = new WhereClause(new Operator.Equals("id", 9));
        await repo.ReadSingle(new QueryStatement(new() { Table = "users", Fields = ["id"], Where = where }));
        Assert.Equal("SELECT id FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        Assert.Null(await repo.ReadSingle());
    }

    [Fact]
    public async Task GetAttachesWhereToDefaultSelect()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([]);
        await repo.ReadSingle(new WhereClause(new Operator.Equals("id", 9)));
        Assert.Equal("SELECT id, name FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        await foreach (var _ in repo.Read(new WhereClause(new Operator.Equals("id", 9))))
            break;
    }

    [Fact]
    public async Task ReadCallsStoredProcedure()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        var call = new ExecuteStoredProcedureStatement("get_user", [new Parameter("id", 9)]);
        var rows = new List<UserRow>();
        await foreach (var row in repo.Read(call))
            rows.Add(row);
        Assert.Equal(1, rows[0].id);
        Assert.Equal("a", rows[0].name);
        Assert.Equal("CALL get_user($id)", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(new Value.Int(9), command.ExecuteReaderCalls[0].Parameters![0].Value);
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Assert.Equal(1, (await repo.ReadSingle(call))!.id);
    }

    [Fact]
    public async Task AddUpdateDeleteUseBuilders()
    {
        var (repo, command) = Create();
        command.Response = new Response(new Value.Int(9), 1);
        var obj = new UserRow { id = 1, name = "a" };
        await repo.Create(obj);
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal(9, obj.id);
        await repo.Update(obj);
        Assert.Equal("UPDATE users SET name = $name WHERE id = $id", command.ExecuteCalls[1].Text);
        Assert.Equal(new Value.Text("a"), command.ExecuteCalls[1].Parameters![0].Value);
        Assert.Equal(new Value.Int(9), command.ExecuteCalls[1].Parameters![1].Value);
        await repo.Delete(obj);
        Assert.Equal("DELETE FROM users WHERE id = $id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public async Task CreateMapperWritesConfiguredId()
    {
        var (repo, command) = Create<Item>(id: ["Id"]);
        command.Response = new Response(new Value.Int(4), 1);
        var row = new Item { Name = "a" };
        await repo.Create(row);
        Assert.Equal(4, row.Id);
        Assert.Contains("INSERT INTO users (Name)", command.ExecuteCalls[0].Text);
    }

    [Fact]
    public async Task CompositeIdOmitsIdsOnCreate()
    {
        var (repo, command) = Create<Composite>(id: ["org_id", "user_id"]);
        await repo.Create(new Composite { org_id = 1, user_id = 2, name = 3 });
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal(new Value.Int(3), command.ExecuteCalls[0].Parameters![0].Value);
        await repo.Update(new Composite { org_id = 1, user_id = 2, name = 4 });
        Assert.Contains("org_id = $org_id AND user_id = $user_id", command.ExecuteCalls[1].Text);
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
        await typed.Create(new Item { Id = 1, Name = "a" });
        Assert.Contains("INSERT INTO users", command2.ExecuteCalls[0].Text);
        var where = new WhereClause(new Operator.Equals("name", "z"));
        await typed.Delete(new Item { Name = "z" }, where);
        Assert.Equal("DELETE FROM users WHERE name = $name", command2.ExecuteCalls[1].Text);
    }

    sealed class Contact
    {
        public string Name { get; set; } = "";
    }

    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public string email { get; set; } = "";
        public string contact_name1 { get; set; } = "";
        public string contact_name2 { get; set; } = "";
        public List<Contact> Contacts { get; set; } = [];
    }

    static TransformMapper<User, User> UserContactsMapper() =>
        new(
            row =>
            {
                row.Contacts =
                [
                    new Contact { Name = row.contact_name1 },
                    new Contact { Name = row.contact_name2 }
                ];
                return row;
            },
            user =>
            {
                user.contact_name1 = user.Contacts.ElementAtOrDefault(0)?.Name ?? "";
                user.contact_name2 = user.Contacts.ElementAtOrDefault(1)?.Name ?? "";
                return user;
            });

    [Fact]
    public async Task GetAppliesMapperAfterRowConverter()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<UserRow>(connection, new CRUDRepositoryOptions<UserRow>
        {
            Entity = "users",
            RowsMapper = new TransformMapper<UserRow, UserRow>(
                row => { row.name = row.name.ToUpperInvariant(); return row; },
                row => { row.name = row.name.ToLowerInvariant(); return row; })
        });
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1, ["name"] = "a" }]);
        Assert.Equal("A", (await repo.ReadSingle())!.name);
    }

    [Fact]
    public async Task GetMapsFlatRowIntoNestedUserContacts()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
        {
            Entity = "users",
            RowsMapper = UserContactsMapper()
        });
        command.Reader = new MockReader<Row>([new Row
        {
            ["id"] = 1,
            ["username"] = "bobby",
            ["email"] = "bob@asdf.com",
            ["contact_name1"] = "bob",
            ["contact_name2"] = "roberta"
        }]);
        var user = await repo.ReadSingle();
        Assert.Equal("bobby", user!.username);
        Assert.Equal("bob@asdf.com", user.email);
        Assert.Equal(["bob", "roberta"], user.Contacts.Select(contact => contact.Name));
        Assert.Equal(
            "SELECT contact_name1, contact_name2, email, id, username FROM users",
            command.ExecuteReaderCalls[0].Text);
        command.Reader = new MockReader<Row>([new Row
        {
            ["id"] = 1,
            ["username"] = "bobby",
            ["email"] = "bob@asdf.com",
            ["contact_name1"] = "bob",
            ["contact_name2"] = "roberta"
        }]);
        var rows = new List<User>();
        await foreach (var row in repo.Read())
            rows.Add(row);
        Assert.Equal(2, rows[0].Contacts.Count);
    }

    [Fact]
    public async Task AddUpdateDeleteReverseMapThenReverseConvert()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
        {
            Entity = "users",
            RowsMapper = UserContactsMapper()
        });
        var user = new User
        {
            id = 1,
            username = "bobby",
            email = "bob@asdf.com",
            Contacts = [new Contact { Name = "bob" }, new Contact { Name = "roberta" }]
        };
        await repo.Create(user);
        Assert.Equal(
            "INSERT INTO users (contact_name1, contact_name2, email, username) VALUES ($contact_name1, $contact_name2, $email, $username)",
            command.ExecuteCalls[0].Text);
        var names = command.ExecuteCalls[0].Parameters!.ToDictionary(p => p.Name, p => p.Value);
        Assert.Equal(new Value.Text("bob"), names["contact_name1"]);
        Assert.Equal(new Value.Text("roberta"), names["contact_name2"]);
        await repo.Update(user);
        Assert.DoesNotContain("Contacts", command.ExecuteCalls[1].Text);
        await repo.Delete(user);
        Assert.Equal("DELETE FROM users WHERE id = $id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public void FromTypeBuildsOptions()
    {
        var connection = new MockConnection();
        var fromId = CreateRepositoryOptions.FromType<Item>(typeof(Item));
        Assert.Equal(new[] { "Id" }, fromId.Ids);
        Assert.Equal(nameof(Item), fromId.Entity);
        Assert.Empty(CreateRepositoryOptions.FromType<Label>().Ids);
        Assert.NotNull(new CRUDRepository<Item>(connection, fromId));
        Assert.NotNull(new CRUDRepository<Item>(connection));
        Assert.NotNull(new ReadonlyRepository<Item>(connection, fromId));
        Assert.NotNull(new ReadonlyRepository<Item>(connection));
        Assert.Same(connection, RepositoryBuilder<Item>.Parts(connection, fromId).Connection);
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
