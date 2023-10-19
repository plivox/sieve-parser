use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::comparators::Comparators;
use crate::grammar::ast::literal::{Literal, LiteralTypes};
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
            match p.as_rule() {
                Rule::comparators => {
                    test_header.comparators = Some(Comparators::from(p));
                }
                Rule::test_header_argument_header => {
                    let l = Literal::from(p.into_inner().next().unwrap());
                    test_header.header = Some(l.inner());
                }
                Rule::test_header_argument_key => {
                    let l = Literal::from(p.into_inner().next().unwrap());
                    // println!("l: {:#?}", l);
                    test_header.key = Some(l.inner());
                }
                _ => {
                    println!("Unknown rule: {:?}", p.as_rule());
                }
            }
        }

        // let inner = pair.into_inner().next().unwrap();
        //
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

        // println!("{:#?}", pair);
        // let mut pairs = pair.into_inner().into_iter();
        // for pair in pairs {
        //     println!("{:#?}", pair.as_rule());
        // }
        // pairs.for_each(|p: Pair<'r, Rule>| {
        //     println!("{:#?}", p);
        // }
        // let ty = if pairs.peek().is_some() {
        //     // Ty::from_pair(
        //     //     expect!(pairs, Rule::type_specifier, "type specifier", span),
        //     //     context,
        //     // )?
        // } else {
        //     // return Err(AstError::new("Unexpected end of tokens.", span));
        // };

        test_header
    }
}
