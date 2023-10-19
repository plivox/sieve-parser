use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::LiteralTypes;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct Comment {
    comment: LiteralTypes,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            comment: LiteralTypes::String(String::from("")),
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for Comment {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let comment = LiteralTypes::String(pair.as_span().as_str().to_string());
        Self { comment }
    }
}
