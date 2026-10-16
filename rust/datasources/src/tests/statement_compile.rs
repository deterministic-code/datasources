fn stmt_text(statement: &dyn crate::Statement) -> String {
    crate::DefaultStatementTextRenderer::instance().render(statement)
}
fn where_sql(clause: &dyn crate::IWhere) -> String {
    crate::DefaultStatementTextRenderer::instance().render_where(clause)
}

use crate::*;

#[test]
fn wraps_query_text_and_an_optional_where() {
    let select = SelectStatement::new(SelectStatementOptions::new("dual", &["1"]));
    let by_id = WhereText::new("id = 1");
    let with_where = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&by_id),
        ..SelectStatementOptions::new("dual", &["1"])
    });
    assert_eq!(stmt_text(&select), "SELECT 1 FROM dual");
    assert!(select.where_clause().is_none());
    assert!(with_where.where_clause().is_some());
    assert_eq!(where_sql(&by_id), "id = 1");
}

#[test]
fn compiles_a_select() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new("users", &["id", "name"])
    });
    assert_eq!(
        stmt_text(&query),
        "SELECT id, name FROM users WHERE id = $id AND age > $age"
    );
    assert!(query.where_clause().is_some());
}

#[test]
fn joins_tables() {
    let where_clause = Where::new(Operator::equals("users.id", 1_i64));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        joins: &[
            Join::inner("orders", "users.id", "orders.user_id"),
            Join::left("payments", "orders.id", "payments.order_id"),
            Join::right("profiles", "users.id", "profiles.user_id"),
        ],
        ..SelectStatementOptions::new("users", &["users.id"])
    });
    assert_eq!(
        stmt_text(&query),
        "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id"
    );
}

#[test]
fn groups_aggregates_orders_and_limits() {
    let count = Aggregate::count("*", Some("n"));
    let total = Aggregate::sum("amount", Some("total"));
    let grouped_fields = ["status", count.as_str(), total.as_str()];
    let where_clause = Where::new(Operator::equals("status", "open"));
    let grouped = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        joins: &[Join::inner(
            "customers",
            "orders.customer_id",
            "customers.id",
        )],
        group_by: &["status"],
        order_by: &[OrderBy::desc("total"), OrderBy::asc("status")],
        limit: Some(10),
        ..SelectStatementOptions::new("orders", &grouped_fields)
    });
    assert_eq!(
        stmt_text(&grouped),
        "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10"
    );
}

#[test]
fn groups_rows() {
    let by_status = SelectStatement::new(SelectStatementOptions {
        group_by: &["id"],
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(stmt_text(&by_status), "SELECT id FROM users GROUP BY id");
}

#[test]
fn aggregates_without_grouping() {
    let min_n = Aggregate::min("n", None);
    let max_n = Aggregate::max("n", None);
    let bound_fields = [min_n.as_str(), max_n.as_str()];
    let bounds = SelectStatement::new(SelectStatementOptions::new("users", &bound_fields));
    assert_eq!(stmt_text(&bounds), "SELECT MIN(n), MAX(n) FROM users");
}

#[test]
fn orders_rows() {
    let ordered = SelectStatement::new(SelectStatementOptions {
        order_by: &[OrderBy::asc("id"), OrderBy::desc("name")],
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(
        stmt_text(&ordered),
        "SELECT id FROM users ORDER BY id ASC, name DESC"
    );
}

#[test]
fn limits_rows() {
    let limited = SelectStatement::new(SelectStatementOptions {
        limit: Some(10),
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(stmt_text(&limited), "SELECT id FROM users LIMIT 10");
}

#[test]
fn compiles_an_insert() {
    let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
    let insert = InsertStatement::new("users", &fields);
    assert_eq!(
        stmt_text(&insert),
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
}

#[test]
fn compiles_an_update() {
    let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    let update = UpdateStatement::new("users", &fields, Some(&where_clause));
    assert_eq!(
        stmt_text(&update),
        "UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age"
    );
    assert!(update.where_clause().is_some());
}

#[test]
fn compiles_a_delete() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    let delete = DeleteStatement::new("users", Some(&where_clause));
    assert_eq!(
        stmt_text(&delete),
        "DELETE FROM users WHERE id = $id AND age > $age"
    );
    assert!(delete.where_clause().is_some());
}

#[test]
fn compiles_a_create_table() {
    let create_table = CreateTableStatement::new(
        "users",
        &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")],
        &[],
    );
    assert_eq!(
        stmt_text(&create_table),
        "CREATE TABLE users (id INTEGER, name TEXT)"
    );
}

#[test]
fn compiles_named_constraints() {
    let named_table = CreateTableStatement::new(
        "orders",
        &[
            Column::new("id", "INTEGER")
                .default(ColumnDefault::new("0", Some("df_id")))
                .not_null(NotNull::new(Some("nn_id"))),
            Column::new("tenant_id", "INTEGER"),
            Column::new("user_id", "INTEGER"),
            Column::new("qty", "INTEGER"),
        ],
        &[
            PrimaryKey::new("pk_orders", &["id", "tenant_id"]),
            ForeignKey::new(
                "fk_orders_user",
                &["user_id"],
                "users",
                &["id"],
                Some(ReferentialAction::Cascade),
                None,
            ),
            UniqueConstraint::new("uq_orders_user", &["user_id"]),
            CheckConstraint::new("chk_orders_qty", "qty > 0"),
        ],
    );
    assert_eq!(
        stmt_text(&named_table),
        "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))"
    );
}

#[test]
fn compiles_a_create_index() {
    let create_index = CreateIndexStatement::new("idx_orders_user", "orders", &["user_id"], false);
    assert_eq!(
        stmt_text(&create_index),
        "CREATE INDEX idx_orders_user ON orders (user_id)"
    );
}

#[test]
fn compiles_a_create_unique_index() {
    let create_index = CreateIndexStatement::new("uq_orders_code", "orders", &["code"], true);
    assert_eq!(
        stmt_text(&create_index),
        "CREATE UNIQUE INDEX uq_orders_code ON orders (code)"
    );
}

#[test]
fn compiles_an_alter_table_add_column() {
    let alter = AlterTableStatement::new(
        "users",
        AlterAction::AddColumn(Column::new("email", "TEXT").not_null(NotNull::new(None))),
    );
    assert_eq!(
        stmt_text(&alter),
        "ALTER TABLE users ADD COLUMN email TEXT NOT NULL"
    );
}

#[test]
fn compiles_an_alter_table_drop_column() {
    let alter = AlterTableStatement::new("users", AlterAction::DropColumn { name: "email".into() });
    assert_eq!(stmt_text(&alter), "ALTER TABLE users DROP COLUMN email");
}

#[test]
fn compiles_an_alter_table_alter_column() {
    let alter = AlterTableStatement::new(
        "users",
        AlterAction::AlterColumn(
            Column::new("age", "INTEGER")
                .default(ColumnDefault::new("0", None))
                .not_null(NotNull::new(None)),
        ),
    );
    assert_eq!(
        stmt_text(&alter),
        "ALTER TABLE users ALTER COLUMN age TYPE INTEGER DEFAULT 0 NOT NULL"
    );
}

#[test]
fn compiles_an_alter_table_rename_column() {
    let alter = AlterTableStatement::new(
        "users",
        AlterAction::RenameColumn {
            from: "email".into(),
            to: "email_address".into(),
        },
    );
    assert_eq!(
        stmt_text(&alter),
        "ALTER TABLE users RENAME COLUMN email TO email_address"
    );
}

#[test]
fn compiles_an_alter_table_rename_table() {
    let alter = AlterTableStatement::new(
        "users",
        AlterAction::RenameTable {
            to: "accounts".into(),
        },
    );
    assert_eq!(stmt_text(&alter), "ALTER TABLE users RENAME TO accounts");
}

#[test]
fn compiles_an_alter_table_add_constraint() {
    let alter = AlterTableStatement::new(
        "orders",
        AlterAction::AddConstraint(UniqueConstraint::new("uq_orders_user", &["user_id"])),
    );
    assert_eq!(
        stmt_text(&alter),
        "ALTER TABLE orders ADD CONSTRAINT uq_orders_user UNIQUE (user_id)"
    );
}

#[test]
fn compiles_an_alter_table_drop_constraint() {
    let alter = AlterTableStatement::new(
        "orders",
        AlterAction::DropConstraint {
            name: "uq_orders_user".into(),
        },
    );
    assert_eq!(
        stmt_text(&alter),
        "ALTER TABLE orders DROP CONSTRAINT uq_orders_user"
    );
}

#[test]
fn compiles_a_drop_index() {
    let drop = DropIndexStatement::new("idx_orders_user", "orders");
    assert_eq!(stmt_text(&drop), "DROP INDEX IF EXISTS idx_orders_user");
}

#[test]
fn compiles_a_drop_trigger() {
    let drop = DropTriggerStatement::new("trg_users_updated", "users");
    assert_eq!(stmt_text(&drop), "DROP TRIGGER IF EXISTS trg_users_updated");
}

#[test]
fn compiles_a_call() {
    let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
    let call = ExecuteStoredProcedureStatement::new("save", &fields);
    assert_eq!(stmt_text(&call), "CALL save($id, $name)");
}

#[test]
fn compiles_a_select_function_call() {
    let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
    let ret = ReturnStatement::new("add", &fields);
    assert_eq!(stmt_text(&ret), "SELECT add($id, $name)");
}

#[test]
fn renders_compare_groups_not_and_in() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    let in_ids = Where::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)]));
    assert_eq!(where_sql(&where_clause), "id = $id AND age > $age");
    assert_eq!(where_sql(&in_ids), "id IN ($id_0, $id_1)");
}

#[test]
fn compare_tree() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    assert_eq!(where_sql(&where_clause), "id = $id AND age > $age");
}

#[test]
fn attaches_where_to_a_query() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new("users", &["id", "name"])
    });
    assert_eq!(
        stmt_text(&query),
        "SELECT id, name FROM users WHERE id = $id AND age > $age"
    );
}
