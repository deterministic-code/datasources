using Datasources;

namespace Datasources.Tests;

public class EnrichedReadonlyRepositoryTests
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
        public string label { get; set; } = "";
    }

    [Fact]
    public async Task GetFillsLookupSelectAndLeavesMissingDefault()
    {
        var parents = new ListReadonlyRepository<Parent>(
            new Parent { id = 1, type_id = 10 },
            new Parent { id = 2, type_id = 99 });
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 10, name = "person" });
        var repo = new EnrichedReadonlyRepository<Parent>(parents);
        repo.AddEnriched("type_name", "type_id", types, "id", "name");
        var items = new List<Parent>();
        await foreach (var item in repo.Get())
            items.Add(item);
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public async Task GetFillsListsAndGetSingleIsEnriched()
    {
        var parents = new ListReadonlyRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var types = new ListReadonlyRepository<TypeRow>(new TypeRow { id = 10, name = "org" });
        var children = new ListReadonlyRepository<Child>(
            new Child { parent_id = 1, label = "a" },
            new Child { parent_id = 1, label = "b" },
            new Child { parent_id = 2, label = "x" });
        var repo = new EnrichedReadonlyRepository<Parent>(parents);
        repo.AddEnriched("type_name", "type_id", types, "id", "name");
        repo.AddEnriched("children", "id", children, "parent_id", many: true);
        var item = await repo.GetSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal(["a", "b"], item.children.Select(child => child.label));
    }
}
