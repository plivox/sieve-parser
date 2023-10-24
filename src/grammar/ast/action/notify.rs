use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionNotify {
    pub from: Option<LiteralTypes>,
    pub importance: Option<LiteralTypes>,
    pub options: Option<LiteralTypes>,
    pub message: Option<LiteralTypes>,
    pub method: Option<LiteralTypes>,
}

impl Default for ActionNotify {
    fn default() -> Self {
        Self {
            from: None,
            importance: None,
            options: None,
            message: None,
            method: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionNotify {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut action_notify = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::action_notify_argument_from => {
                    action_notify.from = Some(Literal::from(inner).inner());
                }
                Rule::action_notify_argument_importance => {
                    action_notify.importance = Some(Literal::from(inner).inner());
                }
                Rule::action_notify_argument_options => {
                    action_notify.options = Some(Literal::from(inner).inner());
                }
                Rule::action_notify_argument_message => {
                    action_notify.message = Some(Literal::from(inner).inner());
                }
                // Before version 06 (RFC 5429), the notify method was a required argument.
                // In version 06 and later, the notify method is optional. If the notify
                // method is not specified, the default method is "mailto".
                Rule::action_notify_argument_method => {
                    action_notify.method = Some(Literal::from(inner).inner());
                }
                Rule::action_notify_method => {
                    action_notify.method = Some(Literal::from(inner).inner());
                }
                _ => {
                    println!("Unknown rule: {:?}", p.as_rule());
                }
            };
        }

        action_notify
    }
}
