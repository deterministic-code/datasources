using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class BatchEnrichmentTests
{
    const int Parents = 100;

    sealed class Tag
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class ParentTag
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public long TagId { get; set; }
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Label { get; set; } = "";
    }

    sealed class Parent
    {
        public long Id { get; set; }
        public string Label { get; set; } = "";
        public List<Child> Children { get; set; } = [];
        public List<Tag> Tags { get; set; } = [];
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly Enrichment ChildrenFill = new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List);

    static readonly Enrichment TagsFill = new("Tags", "Id", "Id", Association: new("ParentId", "TagId"));

    static CRUDRepository<T> Open<T>(IConnection connection) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Ids = ["Id"],
            PropertyMappings = Snake,
        });

    static async Task<IConnection> OpenConnection()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }

    [Fact]
    public async Task DefaultStrategyLinkedIsOneReadPerTable()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var tagsRepo = Open<Tag>(connection);
        var linksRepo = Open<ParentTag>(connection);
        var tag = await tagsRepo.Create(new Tag { Name = "t" });
        for (var i = 0; i < Parents; i++)
        {
            var parent = await parentsRepo.Create(new Parent());
            await linksRepo.Create(new ParentTag { ParentId = parent.Id, TagId = tag.Id });
        }
        var tagReads = new Reads();
        var linkReads = new Reads();
        parentsRepo.AddLinked(
            TagsFill,
            new CountingRepository<Tag>(tagsRepo, tagReads),
            new CountingCRUDRepository<ParentTag>(linksRepo, linkReads),
            new DefaultReadonlyEnrichmentStrategy<Parent>());
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal(Parents, items.Count);
        Assert.All(items, item => Assert.Single(item.Tags));
        Assert.Equal(1, linkReads.Count);
        Assert.Equal(1, tagReads.Count);
    }

    [Fact]
    public async Task CreateResolvesTwentyLinkedNamesWithOneTagLookup()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var tagsRepo = Open<Tag>(connection);
        var linksRepo = Open<ParentTag>(connection);
        var wanted = new List<Tag>();
        for (var i = 0; i < 20; i++)
            wanted.Add(await tagsRepo.Create(new Tag { Name = $"tag{i}" }));
        var reads = new Reads();
        parentsRepo.AddLinked(
            new("Tags", "Id", "Id", "Name", EnrichmentPropertyType.List, Association: new("ParentId", "TagId")),
            new CountingRepository<Tag>(tagsRepo, reads),
            linksRepo);
        var created = await parentsRepo.Create(new Parent
        {
            Tags = wanted.Select(tag => new Tag { Name = tag.Name }).ToList(),
        });
        var linked = (await linksRepo.Read().ToListAsync()).Where(link => link.ParentId == created.Id).Select(link => link.TagId);
        Assert.Equal(wanted.Select(tag => tag.Id).Order(), linked.Order());
        // one batched name lookup, plus one read when the created row is enriched; was 21
        Assert.Equal(2, reads.Count);
    }

    sealed record Fixture(
        IConnection Connection,
        List<Parent> Parents,
        CRUDRepository<Parent> Host,
        CRUDRepository<Child> Children,
        CRUDRepository<ParentTag> Links,
        Reads ChildReads,
        Reads LinkReads);

    static async Task<Fixture> Seed()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var children = Open<Child>(connection);
        var links = Open<ParentTag>(connection);
        var parents = new List<Parent>();
        for (var i = 0; i < Parents; i++)
        {
            var parent = await parentsRepo.Create(new Parent { Label = $"p{i}" });
            await children.Create(new Child { ParentId = parent.Id, Label = $"child{i}" });
            await links.Create(new ParentTag { ParentId = parent.Id, TagId = i + 1 });
            parents.Add(parent);
        }
        return new Fixture(connection, parents, parentsRepo, children, links, new Reads(), new Reads());
    }

    static List<Parent> WithChangedChildren(Fixture f, IReadOnlyList<Child> stored) =>
        f.Parents
            .Select(parent => new Parent
            {
                Id = parent.Id,
                Children = stored
                    .Where(child => child.ParentId == parent.Id)
                    .Select(child => new Child { Id = child.Id, ParentId = child.ParentId, Label = child.Label + "!" })
                    .ToList(),
            })
            .ToList();

    static async Task<List<string>> Labels(CRUDRepository<Child> children) =>
        (await children.Read().ToListAsync()).Select(child => child.Label).Order().ToList();

    [Fact]
    public async Task UpdateChildrenBatchReadsOnceAndMatchesTheDefaultStrategy()
    {
        var batch = await Seed();
        var single = await Seed();
        var batchParents = WithChangedChildren(batch, await batch.Children.Read().ToListAsync());
        var singleParents = WithChangedChildren(single, await single.Children.Read().ToListAsync());
        await new BatchUpdateEnrichmentStrategy<Parent>().UpdateChildrenBatch(
            new CountingCRUDRepository<Child>(batch.Children, batch.ChildReads), ChildrenFill, batchParents);
        Assert.Equal(1, batch.ChildReads.Count);
        foreach (var parent in singleParents)
            await new DefaultUpdateEnrichmentStrategy<Parent>().UpdateChildren(
                new CountingCRUDRepository<Child>(single.Children, single.ChildReads), ChildrenFill, parent);
        Assert.Equal(Parents, single.ChildReads.Count);
        Assert.Equal(await Labels(single.Children), await Labels(batch.Children));
        Assert.All(await Labels(batch.Children), label => Assert.EndsWith("!", label));
    }

    [Fact]
    public async Task UpdateChildrenBatchDeletesChildrenDroppedFromAParent()
    {
        var f = await Seed();
        var emptied = f.Parents.Select(parent => new Parent { Id = parent.Id }).ToList();
        await new BatchUpdateEnrichmentStrategy<Parent>().UpdateChildrenBatch(f.Children, ChildrenFill, emptied);
        Assert.Empty(await f.Children.Read().ToListAsync());
    }

    [Fact]
    public async Task DeleteChildrenBatchReadsOnceAndMatchesTheDefaultStrategy()
    {
        var batch = await Seed();
        var single = await Seed();
        var keep = Parents - 1;
        var nested = 0;
        await new BatchDeleteEnrichmentStrategy<Parent>().DeleteChildrenBatch(
            new CountingCRUDRepository<Child>(batch.Children, batch.ChildReads),
            ChildrenFill,
            batch.Parents.Take(keep).ToList(),
            rows => { nested += rows.Count; return Task.CompletedTask; });
        Assert.Equal(1, batch.ChildReads.Count);
        Assert.Equal(keep, nested);
        foreach (var parent in single.Parents.Take(keep))
            await new CascadeDeleteEnrichmentStrategy<Parent>().DeleteChildren(
                new CountingCRUDRepository<Child>(single.Children, single.ChildReads),
                ChildrenFill,
                parent,
                _ => Task.CompletedTask);
        Assert.Equal(keep, single.ChildReads.Count);
        Assert.Single(await batch.Children.Read().ToListAsync());
        Assert.Equal(await Labels(single.Children), await Labels(batch.Children));
    }

    [Fact]
    public async Task DeleteLinkedBatchReadsOnceAndMatchesTheDefaultStrategy()
    {
        var batch = await Seed();
        var single = await Seed();
        var keep = Parents - 1;
        await new BatchDeleteEnrichmentStrategy<Parent>().DeleteLinkedBatch(
            new CountingCRUDRepository<ParentTag>(batch.Links, batch.LinkReads), TagsFill, batch.Parents.Take(keep).ToList());
        Assert.Equal(1, batch.LinkReads.Count);
        foreach (var parent in single.Parents.Take(keep))
            await new CascadeDeleteEnrichmentStrategy<Parent>().DeleteLinked(
                new CountingCRUDRepository<ParentTag>(single.Links, single.LinkReads), TagsFill, parent);
        Assert.Equal(keep, single.LinkReads.Count);
        Assert.Single(await batch.Links.Read().ToListAsync());
        Assert.Single(await single.Links.Read().ToListAsync());
    }

    [Fact]
    public async Task UpdateLinkedBatchReadsOnceAndMatchesTheDefaultStrategy()
    {
        var batch = await Seed();
        var single = await Seed();
        List<Parent> Wanted(Fixture f) =>
            f.Parents.Select(parent => new Parent { Id = parent.Id, Tags = [new Tag { Id = 1000 }] }).ToList();
        var batchParents = Wanted(batch);
        await new BatchUpdateEnrichmentStrategy<Parent>().UpdateLinkedBatch(
            new CountingCRUDRepository<ParentTag>(batch.Links, batch.LinkReads), TagsFill, batchParents);
        Assert.Equal(1, batch.LinkReads.Count);
        foreach (var parent in Wanted(single))
            await new DefaultUpdateEnrichmentStrategy<Parent>().UpdateLinked(
                new CountingCRUDRepository<ParentTag>(single.Links, single.LinkReads), TagsFill, parent);
        Assert.Equal(Parents, single.LinkReads.Count);
        var batchTags = (await batch.Links.Read().ToListAsync()).Select(link => link.TagId).Order().ToList();
        var singleTags = (await single.Links.Read().ToListAsync()).Select(link => link.TagId).Order().ToList();
        Assert.Equal(singleTags, batchTags);
        Assert.All(batchTags, tag => Assert.Equal(1000, tag));
    }

    [Fact]
    public async Task PersistBulkUpdateChildrenReadsOnceAndDropsEmptiedParent()
    {
        var batch = await Seed();
        var single = await Seed();
        var batchParents = WithChangedChildren(batch, await batch.Children.Read().ToListAsync());
        batchParents[0].Children = [];
        var singleParents = WithChangedChildren(single, await single.Children.Read().ToListAsync());
        singleParents[0].Children = [];
        batch.Host.AddEnriched(ChildrenFill, new CountingCRUDRepository<Child>(batch.Children, batch.ChildReads));
        await new DefaultWriteEnrichmentStrategy<Parent>().PersistBulk(
            new RepositoryTree(new RepositoryTreeNode(batch.Host, "")),
            batchParents,
            WriteMode.Update);
        Assert.Equal(1, batch.ChildReads.Count);
        foreach (var parent in singleParents)
            await new DefaultUpdateEnrichmentStrategy<Parent>().UpdateChildren(
                new CountingCRUDRepository<Child>(single.Children, single.ChildReads), ChildrenFill, parent);
        Assert.Equal(Parents, single.ChildReads.Count);
        Assert.Equal(await Labels(single.Children), await Labels(batch.Children));
        Assert.DoesNotContain(await batch.Children.Read().ToListAsync(), child => child.ParentId == batch.Parents[0].Id);
        Assert.All(await Labels(batch.Children), label => Assert.EndsWith("!", label));
    }

    [Fact]
    public async Task PersistBulkUpdateLinkedReadsOnceAndMatchesTheDefaultStrategy()
    {
        var batch = await Seed();
        var single = await Seed();
        List<Parent> Wanted(Fixture f) =>
            f.Parents.Select(parent => new Parent { Id = parent.Id, Tags = [new Tag { Id = 1000 }] }).ToList();
        batch.Host.AddLinked(
            TagsFill,
            Open<Tag>(batch.Connection),
            new CountingCRUDRepository<ParentTag>(batch.Links, batch.LinkReads));
        await new DefaultWriteEnrichmentStrategy<Parent>().PersistBulk(
            new RepositoryTree(new RepositoryTreeNode(batch.Host, "")),
            Wanted(batch),
            WriteMode.Update);
        Assert.Equal(1, batch.LinkReads.Count);
        foreach (var parent in Wanted(single))
            await new DefaultUpdateEnrichmentStrategy<Parent>().UpdateLinked(
                new CountingCRUDRepository<ParentTag>(single.Links, single.LinkReads), TagsFill, parent);
        Assert.Equal(Parents, single.LinkReads.Count);
        var batchTags = (await batch.Links.Read().ToListAsync()).Select(link => link.TagId).Order().ToList();
        var singleTags = (await single.Links.Read().ToListAsync()).Select(link => link.TagId).Order().ToList();
        Assert.Equal(singleTags, batchTags);
        Assert.All(batchTags, tag => Assert.Equal(1000, tag));
    }

    sealed class Reads
    {
        public int Count;
        public void Add() => Count++;
    }

    class CountingRepository<T>(IReadonlyRepository<T> inner, Reads reads) : IReadonlyRepository<T>
    {
        public Type Type => inner.Type;
        public Schema Schema => inner.Schema;
        public bool IsSchemaRequired => inner.IsSchemaRequired;

        public Task<T?> ReadSingle(
            IStatement? query = null,
            IReadOnlyList<Parameter>? parameters = null,
            ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.ReadSingle(query, parameters, options);
        }

        public Task<T?> ReadSingle(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.ReadSingle(where, options);
        }

        public IAsyncEnumerable<T> Read(
            IStatement? query = null,
            IReadOnlyList<Parameter>? parameters = null,
            ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.Read(query, parameters, options);
        }

        public IAsyncEnumerable<T> Read(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.Read(where, options);
        }
    }

    sealed class CountingCRUDRepository<T>(ICRUDRepository<T> inner, Reads reads) : CountingRepository<T>(inner, reads), ICRUDRepository<T>
        where T : new()
    {
        public IReadOnlyList<string> Ids => inner.Ids;
        public Task<T> Create(T item) => inner.Create(item);
        public Task<Response> Update(T item, IWhere? byWhere = null) => inner.Update(item, byWhere);
        public Task<Response> Delete(T item, IWhere? byWhere = null) => inner.Delete(item, byWhere);
        public Task<T?> ReadSingleById(Value id) => inner.ReadSingleById(id);
        public Task<T?> ReadSingleById(Row ids) => inner.ReadSingleById(ids);
    }
}
