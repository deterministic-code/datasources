using Datasources;

namespace Datasources.Tests;

public class StatementCompileTests
{
    [Fact]
    public void WrapsQueryTextAndAnOptionalWhere()
    {
        var select = new SelectStatement(new() { Table = "dual", Fields = ["1"] });
        var byId = new WhereText("id = 1");
        var withWhere = new SelectStatement(new() { Table = "dual", Fields = ["1"], Where = byId });
        Assert.Equal("SELECT 1 FROM dual", DefaultStatementTextRenderer.Instance.Render(select));
        Assert.Null(select.Where);
        Assert.Same(byId, withWhere.Where);
        Assert.Equal("id = 1", DefaultStatementTextRenderer.Instance.Render(byId));
    }

    [Fact]
    public void CompilesASelect()
    {
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
        Assert.Equal("SELECT id, name FROM users WHERE id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(query));
        Assert.Same(where, query.Where);
    }

    [Fact]
    public void JoinsTables()
    {
        var query = new SelectStatement(new()
        {
            Table = "users",
            Fields = ["users.id"],
            Where = new Where(new Operator.Equals("users.id", 1)),
            Joins =
            [
                Join.Inner("orders", "users.id", "orders.user_id"),
                Join.Left("payments", "orders.id", "payments.order_id"),
                Join.Right("profiles", "users.id", "profiles.user_id"),
            ],
        });
        Assert.Equal(
            "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
            DefaultStatementTextRenderer.Instance.Render(query));
    }

    [Fact]
    public void GroupsAggregatesOrdersAndLimits()
    {
        var grouped = new SelectStatement(new()
        {
            Table = "orders",
            Fields = ["status", Aggregate.Count("*", "n"), Aggregate.Sum("amount", "total")],
            Where = new Where(new Operator.Equals("status", "open")),
            Joins = [Join.Inner("customers", "orders.customer_id", "customers.id")],
            GroupBy = ["status"],
            OrderBy = [OrderBy.Desc("total"), OrderBy.Asc("status")],
            Limit = 10,
        });
        Assert.Equal(
            "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
            DefaultStatementTextRenderer.Instance.Render(grouped));
    }

    [Fact]
    public void GroupsRows()
    {
        var byStatus = new SelectStatement(new() { Table = "users", Fields = ["id"], GroupBy = ["id"] });
        Assert.Equal("SELECT id FROM users GROUP BY id", DefaultStatementTextRenderer.Instance.Render(byStatus));
    }

    [Fact]
    public void AggregatesWithoutGrouping()
    {
        var bounds = new SelectStatement(new() { Table = "users", Fields = [Aggregate.Min("n"), Aggregate.Max("n")] });
        Assert.Equal("SELECT MIN(n), MAX(n) FROM users", DefaultStatementTextRenderer.Instance.Render(bounds));
    }

    [Fact]
    public void OrdersRows()
    {
        var ordered = new SelectStatement(new() { Table = "users", Fields = ["id"], OrderBy = [OrderBy.Asc("id"), OrderBy.Desc("name")] });
        Assert.Equal("SELECT id FROM users ORDER BY id ASC, name DESC", DefaultStatementTextRenderer.Instance.Render(ordered));
    }

    [Fact]
    public void LimitsRows()
    {
        var limited = new SelectStatement(new() { Table = "users", Fields = ["id"], Limit = 10 });
        Assert.Equal("SELECT id FROM users LIMIT 10", DefaultStatementTextRenderer.Instance.Render(limited));
    }

    [Fact]
    public void CompilesAnInsert()
    {
        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var insert = new InsertStatement("users", fields);
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", DefaultStatementTextRenderer.Instance.Render(insert));
    }

    [Fact]
    public void CompilesAnUpdate()
    {
        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        var update = new UpdateStatement("users", fields, where);
        Assert.Equal("UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(update));
        Assert.Same(where, update.Where);
    }

    [Fact]
    public void CompilesADelete()
    {
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        var delete = new DeleteStatement("users", where);
        Assert.Equal("DELETE FROM users WHERE id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(delete));
        Assert.Same(where, delete.Where);
    }

    [Fact]
    public void CompilesACreateTable()
    {
        var createTable = new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("name", "TEXT")]);
        Assert.Equal("CREATE TABLE users (id INTEGER, name TEXT)", DefaultStatementTextRenderer.Instance.Render(createTable));
    }

    [Fact]
    public void CompilesNamedConstraints()
    {
        var namedTable = new CreateTableStatement(
            "orders",
            [
                new Column("id", "INTEGER", new ColumnDefault("0", "df_id"), new NotNull("nn_id")),
                new Column("tenant_id", "INTEGER"),
                new Column("user_id", "INTEGER"),
                new Column("qty", "INTEGER")
            ],
            [
                new PrimaryKey("pk_orders", ["id", "tenant_id"]),
                new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
                new UniqueConstraint("uq_orders_user", ["user_id"]),
                new CheckConstraint("chk_orders_qty", "qty > 0")
            ]);
        Assert.Equal(
            "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))",
            DefaultStatementTextRenderer.Instance.Render(namedTable));
    }

    [Fact]
    public void CompilesACreateIndex()
    {
        var createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
        Assert.Equal("CREATE INDEX idx_orders_user ON orders (user_id)", DefaultStatementTextRenderer.Instance.Render(createIndex));
    }

    [Fact]
    public void CompilesACreateUniqueIndex()
    {
        var createIndex = new CreateIndexStatement("uq_orders_code", "orders", ["code"], true);
        Assert.Equal("CREATE UNIQUE INDEX uq_orders_code ON orders (code)", DefaultStatementTextRenderer.Instance.Render(createIndex));
    }

    [Fact]
    public void CompilesAnAlterTableAddColumn()
    {
        var alter = new AlterTableStatement("users", new AlterTableStatement.AddColumn(new Column("email", "TEXT", NotNull: new NotNull())));
        Assert.Equal("ALTER TABLE users ADD COLUMN email TEXT NOT NULL", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableDropColumn()
    {
        var alter = new AlterTableStatement("users", new AlterTableStatement.DropColumn("email"));
        Assert.Equal("ALTER TABLE users DROP COLUMN email", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableAlterColumn()
    {
        var alter = new AlterTableStatement("users", new AlterTableStatement.AlterColumn(new Column("age", "INTEGER", new ColumnDefault("0"), new NotNull())));
        Assert.Equal("ALTER TABLE users ALTER COLUMN age TYPE INTEGER DEFAULT 0 NOT NULL", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableRenameColumn()
    {
        var alter = new AlterTableStatement("users", new AlterTableStatement.RenameColumn("email", "email_address"));
        Assert.Equal("ALTER TABLE users RENAME COLUMN email TO email_address", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableRenameTable()
    {
        var alter = new AlterTableStatement("users", new AlterTableStatement.RenameTable("accounts"));
        Assert.Equal("ALTER TABLE users RENAME TO accounts", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableAddConstraint()
    {
        var alter = new AlterTableStatement("orders", new AlterTableStatement.AddConstraint(new UniqueConstraint("uq_orders_user", ["user_id"])));
        Assert.Equal("ALTER TABLE orders ADD CONSTRAINT uq_orders_user UNIQUE (user_id)", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesAnAlterTableDropConstraint()
    {
        var alter = new AlterTableStatement("orders", new AlterTableStatement.DropConstraint("uq_orders_user"));
        Assert.Equal("ALTER TABLE orders DROP CONSTRAINT uq_orders_user", DefaultStatementTextRenderer.Instance.Render(alter));
    }

    [Fact]
    public void CompilesADropIndex()
    {
        var drop = new DropIndexStatement("idx_orders_user", "orders");
        Assert.Equal("DROP INDEX IF EXISTS idx_orders_user", DefaultStatementTextRenderer.Instance.Render(drop));
    }

    [Fact]
    public void CompilesADropTrigger()
    {
        var drop = new DropTriggerStatement("trg_users_updated", "users");
        Assert.Equal("DROP TRIGGER IF EXISTS trg_users_updated", DefaultStatementTextRenderer.Instance.Render(drop));
    }

    [Fact]
    public void CompilesACall()
    {
        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var call = new ExecuteStoredProcedureStatement("save", fields);
        Assert.Equal("CALL save($id, $name)", DefaultStatementTextRenderer.Instance.Render(call));
    }

    [Fact]
    public void CompilesASelectFunctionCall()
    {
        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var ret = new ReturnStatement("add", fields);
        Assert.Equal("SELECT add($id, $name)", DefaultStatementTextRenderer.Instance.Render(ret));
    }

    [Fact]
    public void RendersCompareGroupsNotAndIn()
    {
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        var inIds = new Where(new Operator.In("id", [1, 2]));
        Assert.Equal("id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(where));
        Assert.Equal("id IN ($id_0, $id_1)", DefaultStatementTextRenderer.Instance.Render(inIds));
    }

    [Fact]
    public void CompareTree()
    {
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        Assert.Equal("id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(where));
    }

    [Fact]
    public void AttachesWhereToAQuery()
    {
        var where = new Where(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        var query = new SelectStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
        Assert.Equal("SELECT id, name FROM users WHERE id = $id AND age > $age", DefaultStatementTextRenderer.Instance.Render(query));
    }
}
