use crate::*;

#[test]
fn round_trips_the_select_examples() {
    for sql in [
        "SELECT id FROM users WHERE name NOT LIKE $name",
        "SELECT id FROM users WHERE age NOT BETWEEN $age_0 AND $age_1",
        "SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5",
        "SELECT \"user\".id AS user_id FROM \"user\"",
        "SELECT id FROM users WHERE 'a' = 'b'",
        "SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id AND profiles.active = $profiles.active",
        "SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)",
        "SELECT id FROM users CROSS JOIN tags",
        "SELECT id FROM users WHERE id IS NULL",
        "SELECT id FROM users WHERE id IS NOT NULL",
        "SELECT id FROM users WHERE name LIKE $name",
        "SELECT id FROM users WHERE id IN ($id_0, $id_1)",
        "SELECT id FROM users WHERE id NOT IN ($id_0)",
        "SELECT id FROM users WHERE age BETWEEN $age_0 AND $age_1",
        "SELECT id FROM users WHERE 1 = 0",
        "SELECT id, name FROM users WHERE id = $id AND age > $age",
        "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
    ] {
        assert_eq!(StatementParser::parse(sql).unwrap().get_text(), sql);
    }
}

#[test]
fn builds_the_where_tree() {
    let not_like =
        StatementParser::parse("SELECT id FROM users WHERE name NOT LIKE $name").unwrap();
    let clause = not_like
        .where_clause()
        .unwrap()
        .as_any()
        .downcast_ref::<WhereClause>()
        .unwrap();
    assert!(matches!(
        clause.condition,
        Expression::Operator(Operator::NotLike { .. })
    ));

    let exists = StatementParser::parse(
        "SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)",
    )
    .unwrap();
    let clause = exists
        .where_clause()
        .unwrap()
        .as_any()
        .downcast_ref::<WhereClause>()
        .unwrap();
    assert!(matches!(
        clause.condition,
        Expression::Operator(Operator::Exists { .. })
    ));

    let full = StatementParser::parse(
        "SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id",
    )
    .unwrap();
    assert_eq!(full.joins.len(), 1);
    assert_eq!(full.joins[0].kind, JoinKind::Full);

    let cross = StatementParser::parse("SELECT id FROM users CROSS JOIN tags").unwrap();
    assert_eq!(cross.joins.len(), 1);
    assert_eq!(cross.joins[0].kind, JoinKind::Cross);
    assert!(cross.joins[0].on.is_none());

    let shaped = StatementParser::parse(
        "SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5",
    )
    .unwrap();
    assert!(shaped.distinct);
    assert_eq!(shaped.limit, Some(10));
    assert_eq!(shaped.offset, Some(5));
    let having = shaped
        .having
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<WhereClause>()
        .unwrap();
    assert!(matches!(
        having.condition,
        Expression::Operator(Operator::Equals { .. })
    ));
}

#[test]
fn keeps_a_string_beside_a_column_as_its_parameter() {
    let query = StatementParser::parse("SELECT id FROM users WHERE name = 'ada'").unwrap();
    assert_eq!(query.get_text(), "SELECT id FROM users WHERE name = $name");
    let parameters = query.parameters();
    assert_eq!(parameters.len(), 1);
    assert_eq!(parameters[0].value, Value::Text("ada".into()));
}

#[test]
fn evaluates_the_new_predicates() {
    let row = record(&[
        ("name", Value::Text("ada".into())),
        ("age", Value::Int(30)),
        ("id", Value::Null),
    ]);
    let evaluator = StatementEvaluator;
    assert_eq!(
        evaluator.evaluate(
            &Operator::is_null(Expression::Identifier(Identifier::new("id"))).into(),
            &row,
        ),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(&Operator::like("name", "a%").into(), &row),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(&Operator::not_like("name", "a%").into(), &row),
        Value::Bool(false)
    );
    assert_eq!(
        evaluator.evaluate(&Operator::between("age", 18_i64, 40_i64).into(), &row),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(&Operator::not_between("age", 18_i64, 40_i64).into(), &row),
        Value::Bool(false)
    );
    assert_eq!(
        evaluator.evaluate(
            &Operator::not_in("age", vec![Value::Int(1), Value::Int(2)]).into(),
            &row
        ),
        Value::Bool(true)
    );
    assert_eq!(
        WhereClause::new(Operator::not_like("name", "%a")).get_text(),
        "name NOT LIKE $name"
    );
    assert_eq!(
        WhereClause::new(Operator::not_between("age", 1_i64, 2_i64)).get_text(),
        "age NOT BETWEEN $age_0 AND $age_1"
    );
    assert_eq!(
        WhereClause::new(Operator::not_in("id", vec![])).get_text(),
        "1 = 1"
    );
    assert_eq!(
        WhereClause::new(Operator::not(Operator::like("name", "%a"))).get_text(),
        "NOT (name LIKE $name)"
    );
}

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}
