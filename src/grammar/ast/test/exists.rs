use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestExists {
    pub header: Option<LiteralTypes>,
}

impl Default for TestExists {
    fn default() -> Self {
        Self { header: None }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestExists {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let header = Literal::from(pair.into_inner().next().unwrap()).inner();

        Self {
            header: Some(header),
            ..Self::default()
        }
    }
}
