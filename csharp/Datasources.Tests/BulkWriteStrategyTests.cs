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
    public async Task SingleCommandIBulkWriteStrategyWritesOneStatementPerRow()
    {
        var (repo, command) = Repo(new SingleCommandIBulkWriteStrategy<Parent>());
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

    sealed class Child
    {
        public int? id { get; set; }
        public int? parent_id { get; set; }
        public string label { get; set; } = "";
    }

    sealed class Family
    {
        public int? id { get; set; }
        public string label { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    [Fact]
    public async Task BatchPersistsChildrenInOneCommand()
    {
        var (parentsRepo, childrenRepo, command) = FamilyRepos(new BatchCommandIBulkWriteStrategy<Family>());
        await parentsRepo.Create(
        [
            new Family { id = 1, label = "a", children = [new Child { id = 10, label = "x" }] },
            new Family { id = 2, label = "b", children = [new Child { id = 11, label = "y" }] }
        ]);
        Assert.Equal(2, command.ExecuteCalls.Count);
        Assert.Contains("INSERT INTO parents", command.ExecuteCalls[0].Text);
        Assert.Contains("INSERT INTO children", command.ExecuteCalls[1].Text);
        Assert.Contains("label__0", command.ExecuteCalls[1].Text);
        Assert.Contains("label__1", command.ExecuteCalls[1].Text);
        _ = childrenRepo;
    }

    [Fact]
    public async Task StoredProcedurePersistsChildrenInOneCall()
    {
        var (parentsRepo, _, command) = FamilyRepos(new StoredProcedureBulkWriteStrategy<Family>(
            "bulk_create_parent", "bulk_update_parent", "bulk_delete_parent"),
            new StoredProcedureBulkWriteStrategy<Child>("bulk_create_child", "bulk_update_child", "bulk_delete_child"));
        await parentsRepo.Create(
        [
            new Family { id = 1, label = "a", children = [new Child { id = 10, label = "x" }] },
            new Family { id = 2, label = "b", children = [new Child { id = 11, label = "y" }] }
        ]);
        Assert.Equal(2, command.ExecuteCalls.Count);
        Assert.Equal("CALL bulk_create_parent($payload)", command.ExecuteCalls[0].Text);
        Assert.Equal("CALL bulk_create_child($payload)", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task BatchCreateRequiresParentIdWhenChildrenArePresent()
    {
        var (parentsRepo, _, _) = FamilyRepos(new BatchCommandIBulkWriteStrategy<Family>());
        var error = await Assert.ThrowsAsync<DatasourceException>(() => parentsRepo.Create(
        [
            new Family { label = "a", children = [new Child { label = "x" }] }
        ]));
        Assert.Contains("id", error.Message);
    }

    [Fact]
    public async Task EmptyListDoesNotExecute()
    {
        await AssertEmpty(new SingleCommandIBulkWriteStrategy<Parent>());
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
        Assert.Empty(created);
        Assert.Equal(0, updated.RowsAffected);
        Assert.Equal(0, deleted.RowsAffected);
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

    static (BulkCRUDRepository<Family> Parents, BulkCRUDRepository<Child> Children, MockCommand Command) FamilyRepos(
        IBulkWriteStrategy<Family> parents,
        IBulkWriteStrategy<Child>? children = null)
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var childrenRepo = new BulkCRUDRepository<Child>(
            connection,
            new CRUDRepositoryOptions<Child> { Entity = "children" },
            children ?? new BatchCommandIBulkWriteStrategy<Child>());
        var parentsRepo = new BulkCRUDRepository<Family>(
            connection,
            new CRUDRepositoryOptions<Family> { Entity = "parents" },
            parents);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Type: EnrichmentPropertyType.List), childrenRepo);
        return (parentsRepo, childrenRepo, command);
    }
}
