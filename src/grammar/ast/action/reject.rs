use pest::iterators::Pair;
use serde::Serialize;

// use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionReject {}

impl Default for ActionReject {
    fn default() -> Self {
        Self {}
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionReject {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let _ = pair;
        Self::default()

        // let mut test_discard = Self::default();
        // ...
        // test_discard
    }
}
