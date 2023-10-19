use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::{tree, Node};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestAllof {
    pub children: Vec<Box<Node>>,
}

impl Default for TestAllof {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestAllof {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let children = tree(pair.into_inner(), vec![]);

        Self {
            children,
            ..Self::default()
        }
    }
}
