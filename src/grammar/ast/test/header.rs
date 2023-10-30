use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::ast::Comparators;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestHeader {
    pub comparators: Option<Comparators>,
    pub header: Option<LiteralTypes>,
    pub key: Option<LiteralTypes>,
}

impl Default for TestHeader {
    fn default() -> Self {
        Self {
            comparators: None,
            header: None,
            key: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestHeader {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut test_header = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::comparators => {
                    test_header.comparators = Some(Comparators::from(inner));
                }
                Rule::tst_header_arg_header => {
                    test_header.header = Some(Literal::from(inner).inner());
                }
                Rule::tst_header_arg_key => {
                    test_header.key = Some(Literal::from(inner).inner());
                }
                _ => {
                    unreachable!("Unexpected {:?} token inside test header", p.as_rule());
                }
            }
        }

        // pair.into_inner()
        //     .for_each(|p: Pair<'r, Rule>| match p.as_rule() {
        //         Rule::comparators => {
        //             test_header.kind = String::from(format!("{:?}", p.as_rule()));
        //         }
        //         Rule::test_header_argument_header => {
        //             test_header.header = Literal::from(p.into_inner().next().unwrap()).inner();
        //         }
        //         Rule::test_header_argument_key => {
        //             test_header.key = Literal::from(p.into_inner().next().unwrap()).inner();
        //         }
        //         _ => {
        //             println!("Unknown rule: {:?}", p.as_rule());
        //         }
        //     });

        test_header
    }
}
