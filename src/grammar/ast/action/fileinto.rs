use pest::iterators::Pair;
use serde::Serialize;

// use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionFileinto {}

impl Default for ActionFileinto {
    fn default() -> Self {
        Self {}
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionFileinto {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let _ = pair;
        Self::default()

        // let mut test_fileinto = Self::default();
        // ...
        // test_fileinto
    }
}
