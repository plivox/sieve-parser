use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum TestSizeArgument {
    Over,
    Under,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestSize {
    pub argument: TestSizeArgument,
    pub value: LiteralTypes,
    // pub quantifier: LiteralTypes,
}

impl Default for TestSize {
    fn default() -> Self {
        Self {
            argument: TestSizeArgument::Over,
            value: LiteralTypes::Number(0),
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestSize {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut test_size = Self::default();

        for p in pair.into_inner().into_iter() {
            match p.as_rule() {
                Rule::test_size_argument => {
                    test_size.argument = match p.as_str() {
                        "over" => TestSizeArgument::Over,
                        "under" => TestSizeArgument::Under,
                        _ => {
                            unreachable!("Expected pair to be an TestSizeArgument")
                        }
                    }
                }
                Rule::integer => {
                    test_size.value = Literal::from(p).inner();
                }
                Rule::quantifier => {
                    // test_size.quantifier = Literal::from(p).inner();
                }
                _ => {
                    unreachable!("Unexpected {:?} token inside test size", p.as_rule())
                }
            }
        }

        test_size
    }
}
