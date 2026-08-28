namespace Datasources.Builders;

public sealed class WhereBuilder : IWhere
{
    private readonly Node _node;
    private WhereBuilder(Node node) => _node = node;

    public static WhereBuilder Equals(string field, object? value) => Compare(field, "=", value);
    public static WhereBuilder Ne(string field, object? value) => Compare(field, "<>", value);
    public static WhereBuilder GreaterThan(string field, object? value) => Compare(field, ">", value);
    public static WhereBuilder Lt(string field, object? value) => Compare(field, "<", value);
    public static WhereBuilder Gte(string field, object? value) => Compare(field, ">=", value);
    public static WhereBuilder Lte(string field, object? value) => Compare(field, "<=", value);
    public static WhereBuilder And(params WhereBuilder[] children) => new(new Node.Group("AND", "1 = 1", children));
    public static WhereBuilder Or(params WhereBuilder[] children) => new(new Node.Group("OR", "1 = 0", children));
    public static WhereBuilder Not(WhereBuilder child) => new(new Node.Negation(child));

    private static WhereBuilder Compare(string field, string op, object? value) =>
        new(new Node.Comparison(field, op, value));

    public string GetText() => _node.Text();

    private abstract record Node
    {
        public abstract string Text();

        public sealed record Comparison(string Field, string Op, object? Value) : Node
        {
            public override string Text() => $"{Field} {Op} ${Field}";
        }

        public sealed record Group(string Join, string Empty, WhereBuilder[] Children) : Node
        {
            public override string Text() =>
                Children.Length == 0 ? Empty
                : Children.Length == 1 ? Children[0].GetText()
                : string.Join($" {Join} ", Children.Select(Wrap));
        }

        public sealed record Negation(WhereBuilder Child) : Node
        {
            public override string Text() => $"NOT ({Child.GetText()})";
        }

        private static string Wrap(WhereBuilder child) =>
            child._node is Comparison ? child.GetText() : $"({child.GetText()})";
    }
}
