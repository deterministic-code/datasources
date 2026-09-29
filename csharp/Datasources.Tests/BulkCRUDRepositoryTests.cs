using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class BulkCRUDRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
        public string label { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    sealed class Child
    {
        public int id { get; set; }
        public int parent_id { get; set; }
        public string label { get; set; } = "";
    }

    [Fact]
    public async Task CreateWritesParentsAndChildren()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        var response = await Repo(parentsRepo, childrenRepo).Create(
        [
            new Parent { children = [new Child { label = "a" }] },
            new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
        ]);
        var parents = await parentsRepo.Read().ToListAsync();
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, parents.Count);
        Assert.All(parents, parent => Assert.True(parent.id > 0));
        Assert.Equal(3, children.Count);
        Assert.Equal(parents.Select(parent => parent.id), children.Select(child => child.parent_id).Distinct());
        Assert.Equal(2, response.RowsAffected);
        Assert.Equal(new Value.Int(parents.Max(parent => parent.id)), response.Result);
    }

    [Fact]
    public async Task BulkCreateMatchesListCreate()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var response = await parentsRepo.BulkCreate([new Parent { label = "a" }, new Parent { label = "b" }]);
        var parents = await parentsRepo.Read().ToListAsync();
        Assert.Equal(2, parents.Count);
        Assert.Equal(2, response.RowsAffected);
        Assert.Equal(new Value.Int(parents.Max(parent => parent.id)), response.Result);
    }

    [Fact]
    public async Task UpdateReplaceSyncsEachParent()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        var first = await parentsRepo.Create(new Parent { label = "one" });
        Assert.True(first.id > 0);
        var second = await parentsRepo.Create(new Parent { label = "two" });
        Assert.True(second.id > 0);
        var kept = await childrenRepo.Create(new Child { parent_id = first.id, label = "a" });
        Assert.True(kept.id > 0);
        var dropped = await childrenRepo.Create(new Child { parent_id = second.id, label = "b" });
        Assert.True(dropped.id > 0);
        var response = await Repo(parentsRepo, childrenRepo).BulkUpdate(
        [
            new Parent { id = first.id, label = "one", children = [new Child { id = kept.id, label = "aa" }] },
            new Parent { id = second.id, label = "two", children = [new Child { label = "c" }] }
        ]);
        Assert.Equal(2, response.RowsAffected);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.Equal("aa", children.Single(child => child.parent_id == first.id).label);
        Assert.Equal("c", children.Single(child => child.parent_id == second.id).label);
        Assert.DoesNotContain(children, child => child.label == "b");
    }

    [Fact]
    public async Task BatchUpdateKeepsEarlierWritesWhenTransactionIsOff()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var first = await parentsRepo.Create(new Parent { label = "a" });
        Assert.True(first.id > 0);
        var second = await parentsRepo.Create(new Parent { label = "b" });
        Assert.True(second.id > 0);
        parentsRepo.AddMiddleware(new FailAfter());
        await Assert.ThrowsAsync<DatasourceException>(() => parentsRepo.Update(
        [
            new Parent { id = first.id, label = "changed" },
            new Parent { id = first.id, label = "later" }
        ]));
        Assert.Equal("changed", (await parentsRepo.Read().ToListAsync()).Single(parent => parent.id == first.id).label);
    }

    [Fact]
    public async Task BatchUpdateRollsBackWhenTransactionIsOn()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
        {
            Write = new DefaultWriteEnrichmentStrategy<Parent>(useTransaction: true),
        });
        var first = await parentsRepo.Create(new Parent { label = "a" });
        Assert.True(first.id > 0);
        var second = await parentsRepo.Create(new Parent { label = "b" });
        Assert.True(second.id > 0);
        parentsRepo.AddMiddleware(new FailAfter());
        await Assert.ThrowsAsync<DatasourceException>(() => parentsRepo.Update(
        [
            new Parent { id = first.id, label = "changed" },
            new Parent { id = first.id, label = "later" }
        ]));
        Assert.Equal("a", (await parentsRepo.Read().ToListAsync()).Single(parent => parent.id == first.id).label);
    }

    [Fact]
    public async Task DeleteCascadesEachParent()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        var first = await parentsRepo.Create(new Parent { label = "one" });
        Assert.True(first.id > 0);
        var second = await parentsRepo.Create(new Parent { label = "two" });
        Assert.True(second.id > 0);
        var firstChild = await childrenRepo.Create(new Child { parent_id = first.id, label = "a" });
        Assert.True(firstChild.id > 0);
        var secondChild = await childrenRepo.Create(new Child { parent_id = second.id, label = "b" });
        Assert.True(secondChild.id > 0);
        var kept = await childrenRepo.Create(new Child { label = "keep" });
        Assert.True(kept.id > 0);
        var response = await Repo(parentsRepo, childrenRepo).BulkDelete([new Parent { id = first.id }, new Parent { id = second.id }]);
        Assert.Equal(2, response.RowsAffected);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Equal("keep", Assert.Single(await childrenRepo.Read().ToListAsync()).label);
    }

    sealed class FailAfter : ICommandMiddleware
    {
        int seen;

        public Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(
            string text,
            IReadOnlyList<Parameter>? parameters)
        {
            if (++seen == 2)
                throw new DatasourceException("stop");
            return Task.FromResult<(string, IReadOnlyList<Parameter>?)?>(null);
        }

        public Task AfterCommand(
            string text,
            IReadOnlyList<Parameter>? parameters,
            IReadOnlyList<Row> results,
            double elapsedMs,
            Exception? error = null) =>
            Task.CompletedTask;
    }

    static BulkCRUDRepository<Parent> Repo(
        BulkCRUDRepository<Parent> parentsRepo,
        BulkCRUDRepository<Child> childrenRepo)
    {
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        return parentsRepo;
    }

    static async Task<IConnection> Setup()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }
}
