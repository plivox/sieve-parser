use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionFileinto {
    mailbox: Option<LiteralTypes>,
}

impl Default for ActionFileinto {
    fn default() -> Self {
        Self { mailbox: None }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionFileinto {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mailbox = Some(Literal::from(pair.into_inner().next().unwrap()).inner());

        Self {
            mailbox,
            ..Self::default()
        }
    }
}
