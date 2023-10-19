use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::{tree, Node};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestAnyof {
    pub children: Vec<Box<Node>>,
}

impl Default for TestAnyof {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestAnyof {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let children = tree(pair.into_inner(), vec![]);

        Self {
            children,
            ..Self::default()
        }
    }
}
