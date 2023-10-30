use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::ast::Comparators;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestFlag {
    pub comparators: Option<Comparators>,
    pub variable: Option<LiteralTypes>,
    pub flags: Option<LiteralTypes>,
}

impl Default for TestFlag {
    fn default() -> Self {
        Self {
            comparators: None,
            variable: None,
            flags: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestFlag {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut test_flag = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::comparators => {
                    test_flag.comparators = Some(Comparators::from(inner));
                }
                Rule::tst_has_flag_arg_variable => {
                    test_flag.variable = Some(Literal::from(inner).inner());
                }
                Rule::tst_has_flag_arg_flags => {
                    test_flag.flags = Some(Literal::from(inner).inner());
                }
                _ => {
                    unreachable!("Unexpected {:?} token inside test header", p.as_rule());
                }
            }
        }

        test_flag
    }
}
