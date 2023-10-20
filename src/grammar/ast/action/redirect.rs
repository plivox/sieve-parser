use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionRedirect {
    address: Option<LiteralTypes>,
}

impl Default for ActionRedirect {
    fn default() -> Self {
        Self { address: None }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionRedirect {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let address = Some(Literal::from(pair.into_inner().next().unwrap()).inner());

        Self {
            address,
            ..Self::default()
        }
    }
}
