use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

macro_rules! struct_flag {
    ($struct_name:ident ) => {
        #[derive(Debug, Clone, Eq, PartialEq, Serialize)]
        #[serde(tag = "kind")]
        pub struct $struct_name {
            pub variable: Option<LiteralTypes>,
            pub flags: Option<LiteralTypes>,
        }

        impl $struct_name {
            pub fn default() -> $struct_name {
                Self {
                    variable: None,
                    flags: None,
                }
            }
        }

        impl<'r> From<Pair<'r, Rule>> for $struct_name {
            fn from(pair: Pair<'r, Rule>) -> Self {
                let mut action_flag = Self::default();

                for p in pair.into_inner().into_iter() {
                    let inner = match p.clone().into_inner().next() {
                        Some(inner) => inner,
                        None => p.clone(),
                    };

                    match p.as_rule() {
                        Rule::act_flag_arg_variable => {
                            action_flag.variable = Some(Literal::from(inner).inner());
                        }
                        Rule::act_flag_arg_flags => {
                            action_flag.flags = Some(Literal::from(inner).inner());
                        }
                        _ => {
                            println!("Unknown rule: {:?}", p.as_rule());
                        }
                    };
                }

                action_flag
            }
        }
    };
}

struct_flag!(ActionSetFlag);
struct_flag!(ActionAddFlag);
struct_flag!(ActionRemoveFlag);
