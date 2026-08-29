namespace Datasources.Builders;

public sealed class WhereBuilder : IWhere
{
    private readonly Node _node;
    private WhereBuilder(Node node) => _node = node;

    public static WhereBuilder Equals(string field, Value? value) => Compare(field, "=", value);
    public static WhereBuilder Ne(string field, Value? value) => Compare(field, "<>", value);
    public static WhereBuilder GreaterThan(string field, Value? value) => Compare(field, ">", value);
    public static WhereBuilder Lt(string field, Value? value) => Compare(field, "<", value);
    public static WhereBuilder Gte(string field, Value? value) => Compare(field, ">=", value);
    public static WhereBuilder Lte(string field, Value? value) => Compare(field, "<=", value);
    public static WhereBuilder And(params WhereBuilder[] children) => new(new Node.Group("AND", "1 = 1", children));
    public static WhereBuilder Or(params WhereBuilder[] children) => new(new Node.Group("OR", "1 = 0", children));
    public static WhereBuilder Not(WhereBuilder child) => new(new Node.Negation(child));

    private static WhereBuilder Compare(string field, string op, Value? value) =>
        new(new Node.Comparison(field, op, value ?? Value.Null.Instance));

    public string GetText() => _node.Text();
    public IReadOnlyList<Parameter> Parameters => _node.Parameters();

    private abstract record Node
    {
        public abstract string Text();
        public abstract IReadOnlyList<Parameter> Parameters();

        public sealed record Comparison(string Field, string Op, Value Value) : Node
        {
            public override string Text() => $"{Field} {Op} ${Field}";
            public override IReadOnlyList<Parameter> Parameters() => [new(Field, Value)];
        }

        public sealed record Group(string Join, string Empty, WhereBuilder[] Children) : Node
        {
            public override string Text() =>
                Children.Length == 0 ? Empty
                : Children.Length == 1 ? Children[0].GetText()
                : string.Join($" {Join} ", Children.Select(Wrap));
            public override IReadOnlyList<Parameter> Parameters() =>
                Children.SelectMany(child => child.Parameters).ToArray();
        }

        public sealed record Negation(WhereBuilder Child) : Node
        {
            public override string Text() => $"NOT ({Child.GetText()})";
            public override IReadOnlyList<Parameter> Parameters() => Child.Parameters;
        }

        private static string Wrap(WhereBuilder child) =>
            child._node is Comparison ? child.GetText() : $"({child.GetText()})";
    }
}
