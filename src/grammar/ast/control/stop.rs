use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ControlStop {}

impl Default for ControlStop {
    fn default() -> Self {
        Self {}
    }
}

impl<'r> From<Pair<'r, Rule>> for ControlStop {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let _ = pair;
        Self {}
    }
}
