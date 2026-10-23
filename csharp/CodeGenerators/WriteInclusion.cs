namespace Datasources.CodeGenerators;

public static class WriteInclusion
{
    public static (bool Create, bool Update) Resolve(bool? create, bool? update, bool identity, bool fixedId, bool singleGeneratedId) =>
        (
            create ?? DefaultCreate(identity, fixedId, singleGeneratedId),
            update ?? DefaultUpdate(identity, fixedId, singleGeneratedId)
        );

    public static bool Create(SpecTypeField field, SpecType type, DatasourceTable? table) =>
        Of(field, type, table).Create;

    public static bool Update(SpecTypeField field, SpecType type, DatasourceTable? table) =>
        Of(field, type, table).Update;

    public static (bool Create, bool Update) Of(SpecTypeField field, SpecType type, DatasourceTable? table)
    {
        var ids = SpecTables.IdentityColumns(type, table);
        var overlay = table?.Fields.FirstOrDefault(item => item.Name == field.Name);
        var identity = ids.Contains(field.Name);
        var fixedId = overlay?.IsFixedId == true;
        var singleGenerated = identity && ids.Count == 1 && !fixedId;
        return Resolve(overlay?.IncludeInCreate, overlay?.IncludeInUpdate, identity, fixedId, singleGenerated);
    }

    static bool DefaultCreate(bool identity, bool fixedId, bool singleGeneratedId) =>
        !identity || fixedId || !singleGeneratedId;

    static bool DefaultUpdate(bool identity, bool fixedId, bool singleGeneratedId) =>
        !identity || !fixedId && !singleGeneratedId;
}
