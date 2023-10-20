use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::ast::Quantifiers;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestSizeArgument {
    Over,
    Under,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestSize {
    pub argument: Option<TestSizeArgument>,
    pub value: Option<LiteralTypes>,
    pub quantifier: Option<Quantifiers>,
}

impl Default for TestSize {
    fn default() -> Self {
        Self {
            argument: None,
            value: None,
            quantifier: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestSize {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut test_size = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::test_size_argument => {
                    test_size.argument = match p.as_str() {
                        "over" => Some(TestSizeArgument::Over),
                        "under" => Some(TestSizeArgument::Under),
                        _ => {
                            unreachable!("Expected pair to be an TestSizeArgument")
                        }
                    }
                }
                Rule::number => {
                    test_size.value = Some(Literal::from(inner).inner());
                }
                Rule::quantifier => {
                    test_size.quantifier = Some(Quantifiers::from(inner));
                }
                _ => {
                    unreachable!("Unexpected {:?} token inside test size", p.as_rule())
                }
            }
        }

        test_size
    }
}
