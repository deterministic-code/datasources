using Datasources;

namespace Datasources.Tests;

public class BulkWriteStrategyTests
{
    sealed class Parent
    {
        public int? id { get; set; }
        public string label { get; set; } = "";
    }

    sealed class Scalar
    {
        public double amount { get; set; }
        public byte[]? blob { get; set; }
        public long count { get; set; }
        public DateTime? created { get; set; }
        public bool flag { get; set; }
        public string? label { get; set; }
        public decimal? price { get; set; }
        public Guid? token { get; set; }
    }

    [Fact]
    public async Task SingleCommandsWritesOneStatementPerRow()
    {
        var (repo, command) = Repo(new SingleCommands<Parent>());
        command.Response = new Response(new Value.Int(9), 1);
        await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
        await repo.Update([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
        await repo.Delete([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
        Assert.Equal(6, command.ExecuteCalls.Count);
        Assert.Equal("INSERT INTO parents (label) VALUES ($label)", command.ExecuteCalls[0].Text);
        Assert.Equal("INSERT INTO parents (label) VALUES ($label)", command.ExecuteCalls[1].Text);
        Assert.Equal("UPDATE parents SET label = $label WHERE id = $id", command.ExecuteCalls[2].Text);
        Assert.Equal("UPDATE parents SET label = $label WHERE id = $id", command.ExecuteCalls[3].Text);
        Assert.Equal("DELETE FROM parents WHERE id = $id", command.ExecuteCalls[4].Text);
        Assert.Equal("DELETE FROM parents WHERE id = $id", command.ExecuteCalls[5].Text);
    }

    [Fact]
    public async Task BatchCommandIBulkWriteStrategyJoinsStatements()
    {
        var (repo, command) = Repo(new BatchCommandIBulkWriteStrategy<Parent>());
        await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
        Assert.Single(command.ExecuteCalls);
        Assert.Equal(
            "INSERT INTO parents (label) VALUES ($label__0); INSERT INTO parents (label) VALUES ($label__1)",
            command.ExecuteCalls[0].Text);
        Assert.Equal("label__0", command.ExecuteCalls[0].Parameters![0].Name);
        Assert.Equal(new Value.Text("a"), command.ExecuteCalls[0].Parameters![0].Value);
        Assert.Equal("label__1", command.ExecuteCalls[0].Parameters![1].Name);
        Assert.Equal(new Value.Text("b"), command.ExecuteCalls[0].Parameters![1].Value);

        await repo.Update([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
        Assert.Equal(
            "UPDATE parents SET label = $label__0 WHERE id = $id__0; UPDATE parents SET label = $label__1 WHERE id = $id__1",
            command.ExecuteCalls[1].Text);
        Assert.Equal(
            ["label__0", "id__0", "label__1", "id__1"],
            command.ExecuteCalls[1].Parameters!.Select(parameter => parameter.Name));

        await repo.Delete([new Parent { id = 1 }, new Parent { id = 2 }]);
        Assert.Equal(
            "DELETE FROM parents WHERE id = $id__0; DELETE FROM parents WHERE id = $id__1",
            command.ExecuteCalls[2].Text);
        Assert.Equal(["id__0", "id__1"], command.ExecuteCalls[2].Parameters!.Select(parameter => parameter.Name));
    }

    [Fact]
    public async Task StoredProcedureBulkWriteStrategySendsOneJsonCall()
    {
        var (repo, command) = Repo(new StoredProcedureBulkWriteStrategy<Parent>(
            "bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"));
        await repo.Create([new Parent { label = "a" }, new Parent { label = "b" }]);
        Assert.Single(command.ExecuteCalls);
        Assert.Equal("CALL bulk_create_parent($payload)", command.ExecuteCalls[0].Text);
        Assert.Equal("payload", command.ExecuteCalls[0].Parameters![0].Name);
        Assert.Equal(
            new Value.Text("""[{"id":null,"label":"a"},{"id":null,"label":"b"}]"""),
            command.ExecuteCalls[0].Parameters![0].Value);

        await repo.Update([new Parent { id = 1, label = "a" }, new Parent { id = 2, label = "b" }]);
        Assert.Equal("CALL bulk_update_parent($payload)", command.ExecuteCalls[1].Text);
        Assert.Equal(
            new Value.Text("""[{"id":1,"label":"a"},{"id":2,"label":"b"}]"""),
            command.ExecuteCalls[1].Parameters![0].Value);

        await repo.Delete([new Parent { id = 1 }, new Parent { id = 2 }]);
        Assert.Equal("CALL bulk_delete_parent($payload)", command.ExecuteCalls[2].Text);
        Assert.Equal(
            new Value.Text("""[{"id":1,"label":""},{"id":2,"label":""}]"""),
            command.ExecuteCalls[2].Parameters![0].Value);
    }

    [Fact]
    public async Task PayloadEncodesScalars()
    {
        var (repo, command) = Open(new StoredProcedureBulkWriteStrategy<Scalar>("create_scalar", "update_scalar", "delete_scalar"));
        var created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc);
        await repo.Create([new Scalar
        {
            amount = 1.25,
            blob = [1, 2, 3],
            count = 2,
            created = created,
            flag = true,
            price = 10.5m,
            token = Guid.Parse("3f2504e0-4f89-11d3-9a0c-0305e82c3301")
        }]);
        Assert.Equal(
            new Value.Text(
                "[" +
                "{\"amount\":1.25,\"blob\":\"AQID\",\"count\":2,\"created\":\"" + created.ToString("o") + "\"," +
                "\"flag\":true,\"label\":null,\"price\":\"10.5\",\"token\":\"3f2504e0-4f89-11d3-9a0c-0305e82c3301\"}" +
                "]"),
            command.ExecuteCalls[0].Parameters![0].Value);
    }

    [Fact]
    public async Task EmptyListDoesNotExecute()
    {
        await AssertEmpty(new SingleCommands<Parent>());
        await AssertEmpty(new BatchCommandIBulkWriteStrategy<Parent>());
        await AssertEmpty(new StoredProcedureBulkWriteStrategy<Parent>("create_parent", "update_parent", "delete_parent"));
    }

    static async Task AssertEmpty(IBulkWriteStrategy<Parent> strategy)
    {
        var (repo, command) = Repo(strategy);
        var created = await repo.Create([]);
        var updated = await repo.Update([]);
        var deleted = await repo.Delete([]);
        Assert.Empty(command.ExecuteCalls);
        Assert.Equal(0, created.RowsAffected);
        Assert.Equal(0, updated.RowsAffected);
        Assert.Equal(0, deleted.RowsAffected);
        Assert.IsType<Value.Null>(created.Result);
    }

    static (BulkCRUDRepository<Parent> Repo, MockCommand Command) Repo(IBulkWriteStrategy<Parent> strategy) =>
        Open(strategy);

    static (BulkCRUDRepository<T> Repo, MockCommand Command) Open<T>(IBulkWriteStrategy<T> strategy) where T : new()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new BulkCRUDRepository<T>(connection, new CRUDRepositoryOptions<T> { Entity = "parents" }, strategy), command);
    }
}
