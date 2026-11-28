<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->

# Tree and graph views

## Nested object on a view

A singular related type on a `view_type` fills that property from the host foreign key. The inherited table keeps the key column. The expanded view drops that column because the nested object's id is the key. Create and update copy `Parent.Id` onto the missing foreign key. They do not insert the related row. The generated C# view is a `record class` of the remaining base columns plus `Parent`, not `Category : CategoryBase`.

```yaml
- category_base:
    tags: [datasource_type]
    fields:
      - id: { type: integer, is_id: true }
      - parent_id:
          references: category_base.id
          is_nullable: true
      - name: { type: string, size: 128 }
- category:
    tags: [view_type]
    inherits: category_base
    fields:
      - parent:
          type: category
          references: category_base.parent_id
          is_nullable: true
```

Read fills `Parent` with the row whose id equals `parent_id`. One level: `Parent.Parent` stays unset. A null parent leaves the nullable key null. The view type has no `parent_id` property.

## Tree and graph views

A `tree` or `graph` tag on a `view_type` fills nested children on read and writes the same nested object on `Create` / `create` / `create`. The inherited `datasource_type` stays a flat table. `readonly_lookup` on that view does not make the generated class read-only.

A single-parent tree. Roots are rows whose parent key is null.

```yaml
- category:
    tags: [datasource_type]
    fields:
      - id: { type: integer, is_id: true }
      - parent_category_id:
          references: category.id
          is_nullable: true
      - name: { type: string, size: 128 }
- category_tree:
    tags: [view_type, tree]
    inherits: category
    fields:
      - children:
          type: category_tree[]
          references: category.parent_category_id
```

A composite parent matches every key pair.

```yaml
- region_category_tree:
    tags: [view_type, tree]
    inherits: region_category
    fields:
      - children:
          type: region_category_tree[]
          references: [region_category.parent_region_id, region_category.parent_category_id]
```

A graph uses a `many_to_many` junction whose foreign keys both point at the node.

```yaml
- node:
    tags: [datasource_type]
    fields:
      - id: { type: integer, is_id: true }
      - value: { type: string, size: 128 }
- node_edge:
    tags: [datasource_type, many_to_many]
    fields:
      - from_node_id: { references: node.id }
      - to_node_id: { references: node.id }
- node_graph:
    tags: [view_type, graph]
    inherits: node
    fields:
      - edges:
          type: node_graph[]
          references: node_edge.from_node_id
```

`generate all` emits `CRUDRepository` / `CRUDRepository` / `CRUDRepository` for the view. Read uses `TreeReadonlyEnrichmentStrategy` / `TreeReadonlyEnrichmentStrategy` / `TreeReadonlyEnrichmentStrategy` or `GraphReadonlyEnrichmentStrategy` / `GraphReadonlyEnrichmentStrategy` / `GraphReadonlyEnrichmentStrategy` with a flat `CRUDRepository` / `CRUDRepository` / `CRUDRepository` via (same mapping, no nested fills). Write uses `TreeCreateEnrichmentStrategy` / `TreeCreateEnrichmentStrategy` / `TreeCreateEnrichmentStrategy` or `GraphCreateEnrichmentStrategy` / `GraphCreateEnrichmentStrategy` / `GraphCreateEnrichmentStrategy`. Create walks nested names parent-first. Tree Create throws `DatasourceException` / `DatasourceError` / `DatasourceError` when a name is already inserted. Graph Create keeps that node and still writes the edge, so a loop is valid.

#### C#

```csharp
await trees.Create(new CategoryTree
{
    Name = "food",
    Children = [new CategoryTree { Name = "fruit", Children = [new CategoryTree { Name = "apple" }] }],
});
await graphs.Create(new NodeGraph
{
    Value = "core",
    Edges =
    [
        new NodeGraph { Value = "blade", Edges = [new NodeGraph { Value = "core" }] },
        new NodeGraph { Value = "potion" },
    ],
});
```

#### Rust

```rust
trees.create(CategoryTree {
    name: "food".into(),
    children: vec![CategoryTree {
        name: "fruit".into(),
        children: vec![CategoryTree { name: "apple".into(), ..Default::default() }],
        ..Default::default()
    }],
    ..Default::default()
}).await?;
```

#### TypeScript

```typescript
await trees.create(categoryTree({
  name: "food",
  children: [categoryTree({ name: "fruit", children: [categoryTree({ name: "apple" })] })],
}));
```
