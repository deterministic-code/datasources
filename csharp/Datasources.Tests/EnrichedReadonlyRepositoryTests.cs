using Datasources;

namespace Datasources.Tests;

public class ReadonlyRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
        public int type_id { get; set; }
        public string type_name { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    sealed class TypeRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class Child
    {
        public int parent_id { get; set; }
        public int type_id { get; set; }
        public string label { get; set; } = "";
        public string type_name { get; set; } = "";
    }

    [Fact]
    public Task ReadFillsLookupSelectAndLeavesMissingDefault() =>
        FillLookup(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetFillsLookupSelectAndLeavesMissingDefault() =>
        FillLookup(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task FillLookup(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parents = new ListReadonlyRepository<Parent>(
            new Parent { id = 1, type_id = 10 },
            new Parent { id = 2, type_id = 99 });
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 10, name = "person" });
        parents.AddEnriched("type_name", "type_id", types, "id", "name", strategy: strategy);
        var items = new List<Parent>();
        await foreach (var item in parents.Read())
            items.Add(item);
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public Task ReadFillsListsAndReadSingleIsEnriched() =>
        FillLists(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetFillsListsAndReadSingleIsEnriched() =>
        FillLists(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task FillLists(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parents = new ListReadonlyRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 10, name = "org" });
        var children = new ListReadonlyRepository<Child>(
            new Child { parent_id = 1, label = "a" },
            new Child { parent_id = 1, label = "b" },
            new Child { parent_id = 2, label = "x" });
        parents.AddEnriched("type_name", "type_id", types, "id", "name", strategy: strategy);
        parents.AddEnriched("children", "id", children, "parent_id", many: true, strategy: strategy);
        var item = await parents.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal(["a", "b"], item.children.Select(child => child.label));
    }

    [Fact]
    public Task ReadNestsListLookupAndKeepsRootSibling() =>
        NestListLookup(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetNestsListLookupAndKeepsRootSibling() =>
        NestListLookup(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task NestListLookup(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parents = new ListReadonlyRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 10, name = "org" });
        var childTypes = new ListReadonlyRepository<TypeRow>(
            new TypeRow { id = 3, name = "home" });
        var children = new ListReadonlyRepository<Child>(
            new Child { parent_id = 1, type_id = 3, label = "a" },
            new Child { parent_id = 1, type_id = 9, label = "b" });
        children.AddEnriched("type_name", "type_id", childTypes, "id", "name");
        parents.AddEnriched("type_name", "type_id", types, "id", "name", strategy: strategy);
        parents.AddEnriched("children", "id", children, "parent_id", many: true, strategy: strategy);
        var item = await parents.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal("home", item.children[0].type_name);
        Assert.Equal("", item.children[1].type_name);
    }

    [Fact]
    public async Task HashsetUsesOneReadPerEdge()
    {
        var types = new CountingReadonlyRepository<TypeRow>(
            new ListReadonlyRepository<TypeRow>(
                new TypeRow { id = 10, name = "person" },
                new TypeRow { id = 11, name = "org" }));
        var children = new CountingReadonlyRepository<Child>(
            new ListReadonlyRepository<Child>(
                new Child { parent_id = 1, label = "a" },
                new Child { parent_id = 2, label = "b" }));
        var parents = new ListReadonlyRepository<Parent>(
            new Parent { id = 1, type_id = 10 },
            new Parent { id = 2, type_id = 11 });
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
        parents.AddEnriched("type_name", "type_id", types, "id", "name", strategy: strategy);
        parents.AddEnriched("children", "id", children, "parent_id", many: true, strategy: strategy);
        await foreach (var _ in parents.Read()) { }
        Assert.Equal(1, types.Reads);
        Assert.Equal(0, types.ReadSingles);
        Assert.Equal(1, children.Reads);
        Assert.Equal(0, children.ReadSingles);
    }

    [Fact]
    public async Task DefaultUsesReadSinglePerLookupKey()
    {
        var types = new CountingReadonlyRepository<TypeRow>(
            new ListReadonlyRepository<TypeRow>(
                new TypeRow { id = 10, name = "person" },
                new TypeRow { id = 11, name = "org" }));
        var parents = new ListReadonlyRepository<Parent>(
            new Parent { id = 1, type_id = 10 },
            new Parent { id = 2, type_id = 11 });
        parents.AddEnriched("type_name", "type_id", types, "id", "name");
        await foreach (var _ in parents.Read()) { }
        Assert.Equal(2, types.ReadSingles);
        Assert.Equal(0, types.Reads);
    }

    [Fact]
    public async Task FlatToNestedCollapsesJoinRows()
    {
        var rows = new Row[]
        {
            new() { ["id"] = 1, ["type_name"] = "person" },
            new() { ["id"] = 2, ["type_name"] = "org" }
        };
        var hydrate = new TransformMapper<Dictionary<string, MappedValue>, Parent>(
            row => new Parent
            {
                id = row["id"] is MappedValue.Cell(Value.Int(var id)) ? (int)id : 0,
                type_name = row["type_name"] is MappedValue.Cell(Value.Text(var name)) ? name : ""
            },
            _ => throw new InvalidOperationException());
        var repo = new JoinRows(rows, new TransformMapper<IEnumerable<Row>, IReadOnlyList<Parent>>(
            source => new FlatToNestedMapper(new TypeMap
            {
                Key = "id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "id",
                    ["type_name"] = "type_name"
                }
            }).Map(source).Select(hydrate.Map).ToList(),
            _ => throw new InvalidOperationException()));
        var items = new List<Parent>();
        await foreach (var item in repo.Read())
            items.Add(item);
        Assert.Equal([1, 2], items.Select(item => item.id));
        Assert.Equal(["person", "org"], items.Select(item => item.type_name));
    }

    [Fact]
    public async Task LookupKeyComesFromRenamedRow()
    {
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 1, name = "admin" });
        var repo = new SourcedView();
        repo.AddEnriched("UserType", "UserType", types, "id", "name");
        var item = await repo.ReadSingle();
        Assert.Equal("admin", item!.UserType);
    }

    sealed class View
    {
        public string UserType { get; set; } = "";
    }

    sealed class SourcedView : ReadonlyRepository<View>
    {
        protected override async IAsyncEnumerable<(View Item, Row? Source)> ReadItems(
            IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
        {
            await Task.Yield();
            yield return (new View(), new Row { ["UserType"] = Value.From(1) });
        }
    }

    sealed class JoinRows(IReadOnlyList<Row> rows, IMapper<IEnumerable<Row>, IReadOnlyList<Parent>> result)
        : ReadonlyRepository<Parent>(result)
    {
        protected override async IAsyncEnumerable<Row> ReadSources(
            IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
        {
            await Task.Yield();
            foreach (var row in rows)
                yield return row;
        }
    }

    sealed class CountingReadonlyRepository<T>(IReadonlyRepository<T> inner) : ReadonlyRepository<T> where T : new()
    {
        public int Reads { get; private set; }
        public int ReadSingles { get; private set; }

        public override async IAsyncEnumerable<T> Read(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
        {
            Reads++;
            await foreach (var item in inner.Read(query, parameters))
                yield return item;
        }

        protected override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(
            IStatement? query = null, IReadOnlyList<Parameter>? parameters = null) =>
            WithoutSource(Read(query, parameters));

        public override async Task<T?> ReadSingle(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
        {
            ReadSingles++;
            return await inner.ReadSingle(query, parameters);
        }
    }
}
