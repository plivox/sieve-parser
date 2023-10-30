use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

macro_rules! struct_boolean {
    ($struct_name:ident ) => {
        #[derive(Debug, Clone, Eq, PartialEq, Serialize)]
        #[serde(tag = "kind")]
        pub struct $struct_name {
            pub value: Option<LiteralTypes>,
        }

        impl $struct_name {
            pub fn default() -> $struct_name {
                Self { value: None }
            }
        }

        impl<'r> From<Pair<'r, Rule>> for $struct_name {
            fn from(pair: Pair<'r, Rule>) -> Self {
                let value = Some(Literal::from(pair.into_inner().next().unwrap()).inner());
                Self { value }
            }
        }
    };
}

struct_boolean!(TestFalse);
struct_boolean!(TestTrue);
