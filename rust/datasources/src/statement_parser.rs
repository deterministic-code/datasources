use crate::builders::{Join, JoinKind};
use crate::i_where::IWhere;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::statements::statement::Statement;
use crate::statements::text::Text;
use crate::statements::{
    ArrayExpression, DeleteStatement, Expression, Identifier, InsertStatement, Literal, Operator,
    OrderBy, QueryStatementOptions, UpdateStatement, Where,
};
use crate::text_query::QueryStatement;
use crate::value::Value;
use crate::DatasourceError;

pub struct StatementParser;

impl StatementParser {
    pub fn parse(text: &str) -> Result<Box<dyn Statement>> {
        let mut cursor = Cursor::new(text);
        let statement = cursor.statement()?;
        cursor.end()?;
        Ok(statement)
    }
}

struct Cursor<'a> {
    text: &'a str,
    i: usize,
}

impl<'a> Cursor<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, i: 0 }
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>> {
        match self.word() {
            Some("SELECT") => Ok(Box::new(self.select_statement()?)),
            Some("INSERT") => Ok(Box::new(self.parse_insert()?)),
            Some("UPDATE") => Ok(Box::new(self.parse_update()?)),
            Some("DELETE") => Ok(Box::new(self.parse_delete()?)),
            _ => empty(),
        }
    }

    fn word(&mut self) -> Option<&'static str> {
        ["SELECT", "INSERT", "UPDATE", "DELETE"]
            .into_iter()
            .find(|word| self.peek_word(word))
    }

    fn parse_insert(&mut self) -> Result<InsertStatement> {
        let start = self.i;
        self.expect_word("INSERT")?;
        self.expect_word("INTO")?;
        let table = self.ident()?;
        self.expect_char('(')?;
        let mut columns = vec![self.ident()?];
        while self.try_char(',') {
            columns.push(self.ident()?);
        }
        self.expect_char(')')?;
        self.expect_word("VALUES")?;
        self.expect_char('(')?;
        let mut values = vec![self.raw_expr()?];
        while self.try_char(',') {
            values.push(self.raw_expr()?);
        }
        self.expect_char(')')?;
        if values.len() == columns.len()
            && values
                .iter()
                .zip(&columns)
                .all(|(value, column)| value == &format!("${column}"))
        {
            let fields: Vec<Parameter> = columns
                .into_iter()
                .map(|name| Parameter::new(name, Value::Null))
                .collect();
            return Ok(InsertStatement::new(&table, &fields));
        }
        Ok(InsertStatement::text(&self.text[start..self.i]))
    }

    fn parse_update(&mut self) -> Result<UpdateStatement> {
        let start = self.i;
        self.expect_word("UPDATE")?;
        let table = self.ident()?;
        self.expect_word("SET")?;
        let mut columns = Vec::new();
        let mut canonical = self.assignment(&mut columns)?;
        while self.try_char(',') {
            canonical = self.assignment(&mut columns)? && canonical;
        }
        let clause = if self.try_word("WHERE") {
            Some(Where::new(self.expr()?))
        } else {
            None
        };
        if canonical {
            let fields: Vec<Parameter> = columns
                .iter()
                .map(|name| Parameter::new(name.clone(), Value::Null))
                .collect();
            return Ok(UpdateStatement::new(
                &table,
                &fields,
                clause.as_ref().map(|predicate| predicate as &dyn IWhere),
            ));
        }
        Ok(UpdateStatement::text(
            &self.text[start..self.i],
            clause.map(|predicate| Box::new(predicate) as Box<dyn IWhere>),
        ))
    }

    fn assignment(&mut self, columns: &mut Vec<String>) -> Result<bool> {
        let column = self.ident()?;
        self.expect_char('=')?;
        let raw = self.raw_expr()?;
        let canonical = raw == format!("${column}");
        columns.push(column);
        Ok(canonical)
    }

    fn parse_delete(&mut self) -> Result<DeleteStatement> {
        self.expect_word("DELETE")?;
        self.expect_word("FROM")?;
        let table = self.ident()?;
        let clause = if self.try_word("WHERE") {
            Some(Where::new(self.expr()?))
        } else {
            None
        };
        Ok(DeleteStatement::new(
            &table,
            clause.as_ref().map(|predicate| predicate as &dyn IWhere),
        ))
    }

    fn raw_expr(&mut self) -> Result<String> {
        self.skip();
        let start = self.i;
        self.expr()?;
        Ok(self.text[start..self.i].to_string())
    }

    fn end(&mut self) -> Result<()> {
        self.skip();
        if self.i == self.text.len() {
            Ok(())
        } else {
            empty()
        }
    }

    fn select_statement(&mut self) -> Result<QueryStatement> {
        self.expect_word("SELECT")?;
        let distinct = self.try_word("DISTINCT");
        let fields = self.select_list()?;
        self.expect_word("FROM")?;
        let table = self.from_item()?;
        let joins = self.joins()?;
        let where_clause = if self.try_word("WHERE") {
            Some(Where::new(self.expr()?))
        } else {
            None
        };
        let group_by = if self.try_word("GROUP") {
            self.group()?
        } else {
            Vec::new()
        };
        let having = if self.try_word("HAVING") {
            Some(Where::new(self.expr()?))
        } else {
            None
        };
        let order_by = if self.try_word("ORDER") {
            self.order()?
        } else {
            Vec::new()
        };
        let limit = if self.try_word("LIMIT") {
            Some(self.integer()?)
        } else {
            None
        };
        let offset = if self.try_word("OFFSET") {
            Some(self.integer()?)
        } else {
            None
        };
        let field_refs: Vec<&str> = fields.iter().map(String::as_str).collect();
        let group_refs: Vec<&str> = group_by.iter().map(String::as_str).collect();
        let where_ref = where_clause.as_ref().map(|clause| clause as &dyn IWhere);
        let having_ref = having.as_ref().map(|clause| clause as &dyn IWhere);
        let mut statement = QueryStatement::new(QueryStatementOptions {
            distinct,
            where_clause: where_ref,
            joins: &joins,
            group_by: &group_refs,
            having: having_ref,
            order_by: &order_by,
            limit,
            offset,
            ..QueryStatementOptions::new(&table, &field_refs)
        });
        if let Some(clause) = where_clause {
            statement = statement.replace_where(clause);
        }
        if let Some(clause) = having {
            statement = statement.replace_having(clause);
        }
        Ok(statement)
    }

    fn select_list(&mut self) -> Result<Vec<String>> {
        if self.try_char('*') {
            return Ok(Vec::new());
        }
        let mut fields = vec![self.select_item()?];
        while self.try_char(',') {
            fields.push(self.select_item()?);
        }
        Ok(fields)
    }

    fn select_item(&mut self) -> Result<String> {
        for name in ["COUNT", "SUM", "MIN", "MAX"] {
            let mark = self.i;
            if !self.try_word(name) || !self.try_char('(') {
                self.i = mark;
                continue;
            }
            let arg = if self.try_char('*') {
                "*".to_string()
            } else {
                self.ident()?
            };
            self.expect_char(')')?;
            let alias = if self.try_word("AS") {
                format!(" AS {}", self.ident()?)
            } else {
                String::new()
            };
            return Ok(format!("{name}({arg}){alias}"));
        }
        let column = self.ident()?;
        if self.try_word("AS") {
            Ok(format!("{column} AS {}", self.ident()?))
        } else {
            Ok(column)
        }
    }

    fn from_item(&mut self) -> Result<String> {
        if !self.peek('(') {
            return self.ident();
        }
        let nested = self.nested()?;
        self.expect_word("AS")?;
        Ok(format!("({}) AS {}", nested.get_text(), self.ident()?))
    }

    fn nested(&mut self) -> Result<QueryStatement> {
        self.expect_char('(')?;
        let query = self.select_statement()?;
        self.expect_char(')')?;
        Ok(query)
    }

    fn joins(&mut self) -> Result<Vec<Join>> {
        let mut joins = Vec::new();
        while let Some(kind) = self.kind() {
            self.expect_word(match kind {
                JoinKind::Cross => "CROSS",
                JoinKind::Inner => "INNER",
                JoinKind::Left => "LEFT",
                JoinKind::Right => "RIGHT",
                JoinKind::Full => "FULL",
            })?;
            self.expect_word("JOIN")?;
            let table = self.from_item()?;
            joins.push(if kind == JoinKind::Cross {
                Join::cross(table)
            } else {
                Join::condition(kind, table, Where::new(self.on()?).get_text())
            });
        }
        Ok(joins)
    }

    fn kind(&mut self) -> Option<JoinKind> {
        if self.peek_word("CROSS") {
            Some(JoinKind::Cross)
        } else if self.peek_word("INNER") {
            Some(JoinKind::Inner)
        } else if self.peek_word("LEFT") {
            Some(JoinKind::Left)
        } else if self.peek_word("RIGHT") {
            Some(JoinKind::Right)
        } else if self.peek_word("FULL") {
            Some(JoinKind::Full)
        } else {
            None
        }
    }

    fn on(&mut self) -> Result<Expression> {
        self.expect_word("ON")?;
        self.expr()
    }

    fn group(&mut self) -> Result<Vec<String>> {
        self.expect_word("BY")?;
        self.list(|cursor| cursor.ident())
    }

    fn order(&mut self) -> Result<Vec<OrderBy>> {
        self.expect_word("BY")?;
        self.list(|cursor| cursor.order_item())
    }

    fn order_item(&mut self) -> Result<OrderBy> {
        let column = self.ident()?;
        if self.try_word("DESC") {
            Ok(OrderBy::desc(column))
        } else {
            self.expect_word("ASC")?;
            Ok(OrderBy::asc(column))
        }
    }

    fn list<T>(&mut self, mut item: impl FnMut(&mut Self) -> Result<T>) -> Result<Vec<T>> {
        let mut items = vec![item(self)?];
        while self.try_char(',') {
            items.push(item(self)?);
        }
        Ok(items)
    }

    fn expr(&mut self) -> Result<Expression> {
        let mut left = self.and()?;
        while self.try_word("OR") {
            left = Expression::Operator(Operator::or(left, self.and()?));
        }
        Ok(left)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut left = self.not()?;
        while self.try_word("AND") {
            left = Expression::Operator(Operator::and(left, self.not()?));
        }
        Ok(left)
    }

    fn not(&mut self) -> Result<Expression> {
        if !self.peek_word("NOT") || !self.peek_after("NOT", '(') {
            return self.predicate();
        }
        self.expect_word("NOT")?;
        self.expect_char('(')?;
        let inner = self.expr()?;
        self.expect_char(')')?;
        Ok(Expression::Operator(Operator::not(inner)))
    }

    fn predicate(&mut self) -> Result<Expression> {
        if self.try_word("EXISTS") {
            return Ok(Expression::Operator(Operator::exists(
                Expression::Subquery(self.nested()?.get_text()),
            )));
        }
        let left = self.primary()?;
        if self.try_word("IS") {
            return self.null_test(left);
        }
        if self.try_word("NOT") {
            return self.negated(left);
        }
        if self.try_word("IN") {
            return Ok(Expression::Operator(Operator::In {
                left: Box::new(left),
                right: Box::new(self.in_list()?),
            }));
        }
        if self.try_word("BETWEEN") {
            return self.bounds(left, false);
        }
        if self.try_word("LIKE") {
            return Ok(Expression::Operator(Operator::Like {
                left: Box::new(left),
                right: Box::new(self.param()?),
            }));
        }
        if self.try_text("<>") {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::NotEquals { left, right }
            }));
        }
        if self.try_text("<=") {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::LessThanOrEqual { left, right }
            }));
        }
        if self.try_text(">=") {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::GreaterThanOrEqual { left, right }
            }));
        }
        if self.try_char('=') {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::Equals { left, right }
            }));
        }
        if self.try_char('<') {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::LessThan { left, right }
            }));
        }
        if self.try_char('>') {
            return Ok(binary(left, self.primary()?, |left, right| {
                Operator::GreaterThan { left, right }
            }));
        }
        Ok(left)
    }

    fn null_test(&mut self, left: Expression) -> Result<Expression> {
        if self.try_word("NOT") {
            self.expect_word("NULL")?;
            Ok(Expression::Operator(Operator::is_not_null(left)))
        } else {
            self.expect_word("NULL")?;
            Ok(Expression::Operator(Operator::is_null(left)))
        }
    }

    fn negated(&mut self, left: Expression) -> Result<Expression> {
        if self.try_word("IN") {
            return Ok(Expression::Operator(Operator::NotIn {
                left: Box::new(left),
                right: Box::new(self.in_list()?),
            }));
        }
        if self.try_word("BETWEEN") {
            return self.bounds(left, true);
        }
        self.expect_word("LIKE")?;
        Ok(Expression::Operator(Operator::NotLike {
            left: Box::new(left),
            right: Box::new(self.param()?),
        }))
    }

    fn bounds(&mut self, left: Expression, not: bool) -> Result<Expression> {
        let low = self.param()?;
        self.expect_word("AND")?;
        let high = self.param()?;
        Ok(Expression::Operator(if not {
            Operator::NotBetween {
                value: Box::new(left),
                low: Box::new(low),
                high: Box::new(high),
            }
        } else {
            Operator::Between {
                value: Box::new(left),
                low: Box::new(low),
                high: Box::new(high),
            }
        }))
    }

    fn in_list(&mut self) -> Result<Expression> {
        self.expect_char('(')?;
        if self.peek_word("SELECT") {
            let query = self.select_statement()?;
            self.expect_char(')')?;
            return Ok(Expression::Subquery(query.get_text()));
        }
        let mut items = vec![self.param()?];
        while self.try_char(',') {
            items.push(self.param()?);
        }
        self.expect_char(')')?;
        Ok(Expression::Array(ArrayExpression::new(items)))
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.try_char('(') {
            if self.peek_word("SELECT") {
                let query = self.select_statement()?;
                self.expect_char(')')?;
                return Ok(Expression::Subquery(query.get_text()));
            }
            let inner = self.expr()?;
            self.expect_char(')')?;
            return Ok(inner);
        }
        if self.try_char('$') {
            self.ident()?;
            return Ok(Expression::Literal(Literal::new(Value::Null)));
        }
        if self.peek_digit() {
            return Ok(Expression::Literal(Literal::new(self.integer()?)));
        }
        if self.try_word("true") {
            return Ok(Expression::Literal(Literal::new(true)));
        }
        if self.try_word("false") {
            return Ok(Expression::Literal(Literal::new(false)));
        }
        if self.try_word("null") {
            return Ok(Expression::Literal(Literal::new(Value::Null)));
        }
        if self.peek('\'') {
            return self.string();
        }
        Ok(Expression::Identifier(Identifier::new(self.ident()?)))
    }

    fn param(&mut self) -> Result<Expression> {
        self.expect_char('$')?;
        self.ident()?;
        Ok(Expression::Literal(Literal::new(Value::Null)))
    }

    fn string(&mut self) -> Result<Expression> {
        self.expect_char('\'')?;
        let mut value = String::new();
        while self.i < self.text.len() {
            match self.rest().chars().next() {
                Some('\'') if self.rest().chars().nth(1) == Some('\'') => {
                    value.push('\'');
                    self.bump();
                    self.bump();
                }
                Some('\'') => {
                    self.bump();
                    return Ok(Expression::Literal(Literal::new(value)));
                }
                Some(ch) => {
                    value.push(ch);
                    self.bump();
                }
                None => break,
            }
        }
        empty()
    }

    fn ident(&mut self) -> Result<String> {
        let mut name = self.part()?;
        while self.try_char('.') {
            name.push('.');
            name.push_str(&self.part()?);
        }
        Ok(name)
    }

    fn part(&mut self) -> Result<String> {
        self.skip();
        if self.peek('"') {
            self.quoted()
        } else {
            self.name()
        }
    }

    fn quoted(&mut self) -> Result<String> {
        self.expect_char('"')?;
        let mut name = String::from("\"");
        while self.i < self.text.len() {
            match self.rest().chars().next() {
                Some('"') if self.rest().chars().nth(1) == Some('"') => {
                    name.push_str("\"\"");
                    self.bump();
                    self.bump();
                }
                Some('"') => {
                    self.bump();
                    name.push('"');
                    return Ok(name);
                }
                Some(ch) => {
                    name.push(ch);
                    self.bump();
                }
                None => break,
            }
        }
        empty()
    }

    fn name(&mut self) -> Result<String> {
        self.skip();
        let Some(first) = self.rest().chars().next() else {
            return empty();
        };
        if !first.is_ascii_alphabetic() {
            return empty();
        }
        let mut name = String::new();
        name.push(first);
        self.bump();
        while let Some(ch) = self.rest().chars().next() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                self.bump();
            } else {
                break;
            }
        }
        Ok(name)
    }

    fn integer(&mut self) -> Result<i64> {
        self.skip();
        let start = self.i;
        while self
            .rest()
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
        {
            self.bump();
        }
        if start == self.i {
            return empty();
        }
        self.text[start..self.i]
            .parse()
            .map_err(|_| DatasourceError::message("empty statement"))
    }

    fn peek_digit(&mut self) -> bool {
        self.skip();
        self.rest()
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
    }

    fn peek(&mut self, token: char) -> bool {
        self.skip();
        self.rest().chars().next() == Some(token)
    }

    fn peek_word(&mut self, word: &str) -> bool {
        let mark = self.i;
        let matched = self.try_word(word);
        self.i = mark;
        matched
    }

    fn peek_after(&mut self, word: &str, token: char) -> bool {
        let mark = self.i;
        let matched = self.try_word(word) && self.peek(token);
        self.i = mark;
        matched
    }

    fn try_word(&mut self, word: &str) -> bool {
        self.skip();
        let Some(prefix) = self.rest().get(..word.len()) else {
            return false;
        };
        if !prefix.eq_ignore_ascii_case(word) {
            return false;
        }
        let after = self.rest()[word.len()..].chars().next();
        if after.is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            return false;
        }
        self.i += word.len();
        true
    }

    fn try_char(&mut self, token: char) -> bool {
        self.skip();
        if self.rest().chars().next() != Some(token) {
            return false;
        }
        self.bump();
        true
    }

    fn try_text(&mut self, token: &str) -> bool {
        self.skip();
        if self.rest().get(..token.len()) != Some(token) {
            return false;
        }
        self.i += token.len();
        true
    }

    fn expect_char(&mut self, token: char) -> Result<()> {
        if self.try_char(token) {
            Ok(())
        } else {
            empty()
        }
    }

    fn expect_word(&mut self, word: &str) -> Result<()> {
        if self.try_word(word) {
            Ok(())
        } else {
            empty()
        }
    }

    fn skip(&mut self) {
        while self.rest().chars().next().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    fn rest(&self) -> &'a str {
        &self.text[self.i..]
    }

    fn bump(&mut self) {
        if let Some(ch) = self.rest().chars().next() {
            self.i += ch.len_utf8();
        }
    }
}

fn binary(
    left: Expression,
    right: Expression,
    build: impl FnOnce(Box<Expression>, Box<Expression>) -> Operator,
) -> Expression {
    Expression::Operator(build(Box::new(left), Box::new(right)))
}

fn empty<T>() -> Result<T> {
    Err(DatasourceError::message("empty statement"))
}
