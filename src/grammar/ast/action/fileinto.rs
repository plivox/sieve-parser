use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionFileinto {
    copy: Option<bool>,
    create: Option<bool>,
    mailbox: Option<LiteralTypes>,
    flags: Option<LiteralTypes>,
}

impl Default for ActionFileinto {
    fn default() -> Self {
        Self {
            copy: None,
            create: None,
            mailbox: None,
            flags: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionFileinto {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut action_fileinto = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::act_fileinto_arg_copy => {
                    action_fileinto.copy = Some(true);
                }
                Rule::act_fileinto_arg_create => {
                    action_fileinto.create = Some(true);
                }
                Rule::act_fileinto_arg_flags => {
                    action_fileinto.flags = Some(Literal::from(inner).inner());
                }
                Rule::act_fileinto_mailbox => {
                    action_fileinto.mailbox = Some(Literal::from(inner).inner());
                }
                _ => {
                    println!("Unknown rule: {:?}", p.as_rule());
                }
            };
        }

        action_fileinto
    }
}
