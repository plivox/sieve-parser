use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::{
    ast::literal::{Literal, LiteralTypes},
    parser::Rule,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ControlRequire {
    pub capabilities: Option<LiteralTypes>,
}

impl Default for ControlRequire {
    fn default() -> Self {
        Self { capabilities: None }
    }
}

impl<'r> From<Pair<'r, Rule>> for ControlRequire {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let capabilities = Some(Literal::from(pair.into_inner().next().unwrap()).inner());

        Self {
            capabilities,
            ..Self::default()
        }
    }
}
