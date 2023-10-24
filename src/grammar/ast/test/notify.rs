use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::ast::Quantifiers;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestValidNotifyMethod {}

impl Default for TestValidNotifyMethod {
    fn default() -> Self {
        Self {}
    }
}

impl<'r> From<Pair<'r, Rule>> for TestValidNotifyMethod {
    fn from(pair: Pair<'r, Rule>) -> Self {}
}
