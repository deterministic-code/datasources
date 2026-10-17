namespace Datasources.CodeGenerators;

public sealed record ProcedureSpec(string Op, string Name, IReadOnlyList<string> ByProperties);