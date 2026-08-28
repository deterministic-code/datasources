use crate::statement::Statement;
use crate::value::Value;
use crate::where_trait::Where;

#[derive(Clone)]
enum Node {
    Compare {
        field: String,
        op: &'static str,
        #[allow(dead_code)]
        value: Value,
    },
    Group {
        join: &'static str,
        empty: &'static str,
        children: Vec<WhereBuilder>,
    },
    Not {
        child: Box<WhereBuilder>,
    },
}

#[derive(Clone)]
pub struct WhereBuilder {
    node: Node,
}

impl WhereBuilder {
    pub fn equals(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, "=", value)
    }

    pub fn ne(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, "<>", value)
    }

    pub fn greater_than(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, ">", value)
    }

    pub fn lt(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, "<", value)
    }

    pub fn gte(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, ">=", value)
    }

    pub fn lte(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::compare(field, "<=", value)
    }

    pub fn and(children: Vec<WhereBuilder>) -> Self {
        Self {
            node: Node::Group {
                join: "AND",
                empty: "1 = 1",
                children,
            },
        }
    }

    pub fn or(children: Vec<WhereBuilder>) -> Self {
        Self {
            node: Node::Group {
                join: "OR",
                empty: "1 = 0",
                children,
            },
        }
    }

    pub fn not(child: WhereBuilder) -> Self {
        Self {
            node: Node::Not {
                child: Box::new(child),
            },
        }
    }

    fn compare(field: impl Into<String>, op: &'static str, value: impl Into<Value>) -> Self {
        Self {
            node: Node::Compare {
                field: field.into(),
                op,
                value: value.into(),
            },
        }
    }

    fn wrap(child: &WhereBuilder) -> String {
        match child.node {
            Node::Compare { .. } => child.get_text(),
            _ => format!("({})", child.get_text()),
        }
    }
}

impl Statement for WhereBuilder {
    fn get_text(&self) -> String {
        match &self.node {
            Node::Compare { field, op, .. } => format!("{field} {op} ${field}"),
            Node::Not { child } => format!("NOT ({})", child.get_text()),
            Node::Group {
                join,
                empty,
                children,
            } => match children.as_slice() {
                [] => (*empty).to_string(),
                [only] => only.get_text(),
                _ => children
                    .iter()
                    .map(Self::wrap)
                    .collect::<Vec<_>>()
                    .join(&format!(" {join} ")),
            },
        }
    }
}

impl Where for WhereBuilder {}
