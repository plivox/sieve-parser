use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionReject {
    pub sending: Option<LiteralTypes>,
    pub reason: Option<LiteralTypes>,
}

impl Default for ActionReject {
    fn default() -> Self {
        Self {
            sending: None,
            reason: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionReject {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut action_reject = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::action_reject_or_ereject => {
                    action_reject.sending = match p.as_span().as_str() {
                        "reject" => Some(LiteralTypes::String("MDN".to_string())),
                        "ereject" => Some(LiteralTypes::String("DSN".to_string())),
                        _ => None,
                    }
                }
                Rule::action_reject_reason => {
                    action_reject.reason = Some(Literal::from(inner).inner());
                }
                _ => {
                    println!("Unknown rule: {:?}", p.as_rule());
                }
            };
        }

        action_reject
    }
}
