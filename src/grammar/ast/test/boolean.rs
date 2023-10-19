use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestFalse {
    value: LiteralTypes,
}

impl Default for TestFalse {
    fn default() -> Self {
        Self {
            value: LiteralTypes::Boolean(true),
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestFalse {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let value = Literal::from(pair.into_inner().next().unwrap()).inner();
        Self { value }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct TestTrue {
    value: LiteralTypes,
}

impl Default for TestTrue {
    fn default() -> Self {
        Self {
            value: LiteralTypes::Boolean(true),
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for TestTrue {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let value = Literal::from(pair.into_inner().next().unwrap()).inner();
        Self { value }
    }
}

// impl Boolean<'_> for TestTrue {}

// pub trait Alias: One + Two {}

// impl<T> Alias for T where T: One + Two {}

// impl<T: From> From for T {
//     fn my_extension(self) -> Self {
//       println!("Hey, it worked!");
//       self
//     }
//   }

// pub trait Boolean<'r>: From<Pair<'r, Rule>>
// where
//     // Self: From<Pair<'r, Rule>> + Sized,
//     Self: Sized,
// {
//     // fn from(pair: Pair<'r, Rule>) -> Self {
//     //     let value = Literal::from(pair.into_inner().next().unwrap()).inner();
//     //     Self { value }
//     // }

//     // fn from(pair: Pair<'r, Rule>) -> Self {
//     //     let value = Literal::from(pair.into_inner().next().unwrap()).inner();
//     //     Self { value }
//     // }

//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// pub trait Boolean<'r>: From<Pair<'r, Rule>> {
//     fn from(pair: Pair<'r, Rule>) -> Self {
//         let value = Literal::from(pair.into_inner().next().unwrap()).inner();
//         Self { value }
//     }

//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }
