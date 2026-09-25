namespace Datasources;

public static class StatementParser
{
    public static IStatement Parse(string text)
    {
        var cursor = new Cursor(text);
        var statement = cursor.Statement();
        cursor.End();
        return statement;
    }

    sealed class Cursor(string text)
    {
        int i;

        public IStatement Statement() => Word() switch
        {
            "SELECT" => SelectStatement(),
            "INSERT" => ParseInsert(),
            "UPDATE" => ParseUpdate(),
            "DELETE" => ParseDelete(),
            _ => throw new InvalidOperationException("empty statement")
        };

        string? Word() =>
            PeekWord("SELECT") ? "SELECT"
            : PeekWord("INSERT") ? "INSERT"
            : PeekWord("UPDATE") ? "UPDATE"
            : PeekWord("DELETE") ? "DELETE"
            : null;

        InsertStatement ParseInsert()
        {
            var start = i;
            ExpectWord("INSERT");
            ExpectWord("INTO");
            var table = Ident();
            Expect('(');
            var columns = List(Ident);
            Expect(')');
            ExpectWord("VALUES");
            Expect('(');
            var values = new List<string> { RawExpr() };
            while (Try(',')) values.Add(RawExpr());
            Expect(')');
            return values.Count == columns.Count && values.Zip(columns).All(pair => pair.First == "$" + pair.Second)
                ? new InsertStatement(table, columns.Select(name => new Parameter(name, Value.Null.Instance)).ToList())
                : new InsertStatement(text[start..i]);
        }

        UpdateStatement ParseUpdate()
        {
            var start = i;
            ExpectWord("UPDATE");
            var table = Ident();
            ExpectWord("SET");
            var columns = new List<string>();
            var canonical = Assignment(columns);
            while (Try(',')) canonical = Assignment(columns) && canonical;
            IWhere? where = TryWord("WHERE") ? new WhereClause(Expr()) : null;
            return canonical
                ? new UpdateStatement(table, columns.Select(name => new Parameter(name, Value.Null.Instance)).ToList(), where)
                : new UpdateStatement(text[start..i], where);
        }

        bool Assignment(List<string> columns)
        {
            var column = Ident();
            Expect('=');
            var raw = RawExpr();
            columns.Add(column);
            return raw == "$" + column;
        }

        DeleteStatement ParseDelete()
        {
            ExpectWord("DELETE");
            ExpectWord("FROM");
            var table = Ident();
            return new DeleteStatement(table, TryWord("WHERE") ? new WhereClause(Expr()) : null);
        }

        string RawExpr()
        {
            Skip();
            var start = i;
            Expr();
            return text[start..i];
        }

        public void End()
        {
            Skip();
            if (i != text.Length) throw new InvalidOperationException("empty statement");
        }

        QueryStatement SelectStatement()
        {
            ExpectWord("SELECT");
            var distinct = TryWord("DISTINCT");
            var fields = SelectList();
            ExpectWord("FROM");
            var table = FromItem();
            var joins = Joins();
            IWhere? where = TryWord("WHERE") ? new WhereClause(Expr()) : null;
            IReadOnlyList<string>? group = TryWord("GROUP") ? Group() : null;
            IWhere? having = TryWord("HAVING") ? new WhereClause(Expr()) : null;
            IReadOnlyList<OrderBy>? order = TryWord("ORDER") ? Order() : null;
            int? limit = TryWord("LIMIT") ? Integer() : null;
            int? offset = TryWord("OFFSET") ? Integer() : null;
            return new QueryStatement(new QueryStatementOptions
            {
                Distinct = distinct,
                Table = table,
                Fields = fields,
                Where = where,
                Joins = joins,
                GroupBy = group,
                Having = having,
                OrderBy = order,
                Limit = limit,
                Offset = offset
            });
        }

        IReadOnlyList<string> SelectList()
        {
            if (Try('*')) return [];
            var fields = new List<string> { SelectItem() };
            while (Try(',')) fields.Add(SelectItem());
            return fields;
        }

        string SelectItem()
        {
            foreach (var name in new[] { "COUNT", "SUM", "MIN", "MAX" })
            {
                var mark = i;
                if (!TryWord(name) || !Try('('))
                {
                    i = mark;
                    continue;
                }
                var arg = Try('*') ? "*" : Ident();
                Expect(')');
                var alias = TryWord("AS") ? $" AS {Ident()}" : "";
                return $"{name}({arg}){alias}";
            }
            var column = Ident();
            return TryWord("AS") ? $"{column} AS {Ident()}" : column;
        }

        string FromItem()
        {
            if (!Peek('(')) return Ident();
            var nested = Nested();
            ExpectWord("AS");
            return $"({nested.GetText()}) AS {Ident()}";
        }

        QueryStatement Nested()
        {
            Expect('(');
            var query = SelectStatement();
            Expect(')');
            return query;
        }

        IReadOnlyList<Join> Joins()
        {
            var joins = new List<Join>();
            while (Kind() is { } kind)
            {
                ExpectWord(kind switch
                {
                    JoinKind.Cross => "CROSS",
                    JoinKind.Inner => "INNER",
                    JoinKind.Left => "LEFT",
                    JoinKind.Right => "RIGHT",
                    JoinKind.Full => "FULL",
                    _ => throw new InvalidOperationException("empty join")
                });
                ExpectWord("JOIN");
                var table = FromItem();
                joins.Add(kind == JoinKind.Cross
                    ? Join.Cross(table)
                    : new Join(kind, table, On()));
            }
            return joins;
        }

        JoinKind? Kind() =>
            PeekWord("CROSS") ? JoinKind.Cross
            : PeekWord("INNER") ? JoinKind.Inner
            : PeekWord("LEFT") ? JoinKind.Left
            : PeekWord("RIGHT") ? JoinKind.Right
            : PeekWord("FULL") ? JoinKind.Full
            : null;

        IExpression On()
        {
            ExpectWord("ON");
            return Expr();
        }

        IReadOnlyList<string> Group()
        {
            ExpectWord("BY");
            return List(Ident);
        }

        IReadOnlyList<OrderBy> Order()
        {
            ExpectWord("BY");
            return List(OrderItem);
        }

        OrderBy OrderItem()
        {
            var column = Ident();
            if (TryWord("DESC")) return OrderBy.Desc(column);
            ExpectWord("ASC");
            return OrderBy.Asc(column);
        }

        IReadOnlyList<T> List<T>(Func<T> item)
        {
            var items = new List<T> { item() };
            while (Try(',')) items.Add(item());
            return items;
        }

        IExpression Expr()
        {
            var left = And();
            while (TryWord("OR")) left = new Operator.Or(left, And());
            return left;
        }

        IExpression And()
        {
            var left = Not();
            while (TryWord("AND")) left = new Operator.And(left, Not());
            return left;
        }

        IExpression Not()
        {
            if (!PeekWord("NOT") || !PeekAfter("NOT", '(')) return Predicate();
            ExpectWord("NOT");
            Expect('(');
            var inner = Expr();
            Expect(')');
            return new Operator.Not(inner);
        }

        IExpression Predicate()
        {
            if (TryWord("EXISTS")) return new Operator.Exists(new Subquery(Nested()));
            var left = Primary();
            if (TryWord("IS")) return NullTest(left);
            if (TryWord("NOT")) return Negated(left);
            if (TryWord("IN")) return new Operator.In(left, InList());
            if (TryWord("BETWEEN")) return Bounds(left, not: false);
            if (TryWord("LIKE")) return new Operator.Like(left, Param());
            if (Try("<>")) return new Operator.NotEquals(left, Primary());
            if (Try("<=")) return new Operator.LessThanOrEqual(left, Primary());
            if (Try(">=")) return new Operator.GreaterThanOrEqual(left, Primary());
            if (Try('=')) return new Operator.Equals(left, Primary());
            if (Try('<')) return new Operator.LessThan(left, Primary());
            if (Try('>')) return new Operator.GreaterThan(left, Primary());
            return left;
        }

        IExpression NullTest(IExpression left)
        {
            if (TryWord("NOT"))
            {
                ExpectWord("NULL");
                return new Operator.IsNotNull(left);
            }
            ExpectWord("NULL");
            return new Operator.IsNull(left);
        }

        IExpression Negated(IExpression left) =>
            TryWord("IN") ? new Operator.NotIn(left, InList())
            : TryWord("BETWEEN") ? Bounds(left, not: true)
            : Like(left);

        IExpression Like(IExpression left)
        {
            ExpectWord("LIKE");
            return new Operator.NotLike(left, Param());
        }

        IExpression Bounds(IExpression left, bool not)
        {
            var low = Param();
            ExpectWord("AND");
            var high = Param();
            return not
                ? new Operator.NotBetween(left, low, high)
                : new Operator.Between(left, low, high);
        }

        IExpression InList()
        {
            Expect('(');
            if (PeekWord("SELECT"))
            {
                var query = SelectStatement();
                Expect(')');
                return new Subquery(query);
            }
            var items = new List<IExpression> { Param() };
            while (Try(',')) items.Add(Param());
            Expect(')');
            return new ArrayExpression(items);
        }

        IExpression Primary()
        {
            if (Try('('))
            {
                if (PeekWord("SELECT"))
                {
                    var query = SelectStatement();
                    Expect(')');
                    return new Subquery(query);
                }
                var inner = Expr();
                Expect(')');
                return inner;
            }
            if (Try('$'))
            {
                Ident();
                return new Literal(Value.Null.Instance);
            }
            if (PeekDigit()) return new Literal(Integer());
            if (TryWord("true")) return new Literal(true);
            if (TryWord("false")) return new Literal(false);
            if (TryWord("null")) return new Literal(Value.Null.Instance);
            if (Peek('\'')) return String();
            return new Identifier(Ident());
        }

        IExpression Param()
        {
            Expect('$');
            Ident();
            return new Literal(Value.Null.Instance);
        }

        IExpression String()
        {
            Expect('\'');
            var value = "";
            while (i < text.Length)
            {
                if (text[i] == '\'')
                {
                    if (i + 1 < text.Length && text[i + 1] == '\'')
                    {
                        value += "'";
                        i += 2;
                        continue;
                    }
                    i++;
                    return new Literal(value);
                }
                value += text[i++];
            }
            throw new InvalidOperationException("empty statement");
        }

        string Ident()
        {
            var name = Part();
            while (Try('.')) name += "." + Part();
            return name;
        }

        string Part()
        {
            Skip();
            return Peek('"') ? Quoted() : Name();
        }

        string Quoted()
        {
            Expect('"');
            var name = "\"";
            while (i < text.Length)
            {
                if (text[i] == '"')
                {
                    if (i + 1 < text.Length && text[i + 1] == '"')
                    {
                        name += "\"\"";
                        i += 2;
                        continue;
                    }
                    i++;
                    return name + "\"";
                }
                name += text[i++];
            }
            throw new InvalidOperationException("empty statement");
        }

        string Name()
        {
            Skip();
            if (i >= text.Length || !char.IsLetter(text[i]))
                throw new InvalidOperationException("empty statement");
            var start = i++;
            while (i < text.Length && (char.IsLetterOrDigit(text[i]) || text[i] == '_')) i++;
            return text[start..i];
        }

        int Integer()
        {
            Skip();
            var start = i;
            while (i < text.Length && char.IsDigit(text[i])) i++;
            if (start == i || !int.TryParse(text[start..i], out var number))
                throw new InvalidOperationException("empty statement");
            return number;
        }

        bool PeekDigit()
        {
            Skip();
            return i < text.Length && char.IsDigit(text[i]);
        }

        bool Peek(char token)
        {
            Skip();
            return i < text.Length && text[i] == token;
        }

        bool PeekWord(string word)
        {
            var mark = i;
            var matched = TryWord(word);
            i = mark;
            return matched;
        }

        bool PeekAfter(string word, char token)
        {
            var mark = i;
            var matched = TryWord(word) && Peek(token);
            i = mark;
            return matched;
        }

        bool TryWord(string word)
        {
            Skip();
            if (i + word.Length > text.Length) return false;
            if (!text.AsSpan(i, word.Length).Equals(word, StringComparison.OrdinalIgnoreCase)) return false;
            var after = i + word.Length;
            if (after < text.Length && (char.IsLetterOrDigit(text[after]) || text[after] == '_')) return false;
            i = after;
            return true;
        }

        bool Try(char token)
        {
            Skip();
            if (i >= text.Length || text[i] != token) return false;
            i++;
            return true;
        }

        bool Try(string token)
        {
            Skip();
            if (!text.AsSpan(i).StartsWith(token)) return false;
            i += token.Length;
            return true;
        }

        void Expect(char token)
        {
            if (!Try(token)) throw new InvalidOperationException("empty statement");
        }

        void ExpectWord(string word)
        {
            if (!TryWord(word)) throw new InvalidOperationException("empty statement");
        }

        void Skip()
        {
            while (i < text.Length && char.IsWhiteSpace(text[i])) i++;
        }
    }
}
