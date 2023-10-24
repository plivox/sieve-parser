use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::ast::literal::{Literal, LiteralTypes};
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub struct ActionVacation {
    pub days: Option<LiteralTypes>,
    pub subject: Option<LiteralTypes>,
    pub from: Option<LiteralTypes>,
    pub addresses: Option<LiteralTypes>,
    pub mime: Option<LiteralTypes>,
    pub handle: Option<LiteralTypes>,
    pub reason: Option<LiteralTypes>,
}

impl Default for ActionVacation {
    fn default() -> Self {
        Self {
            days: None,
            subject: None,
            from: None,
            addresses: None,
            mime: None,
            handle: None,
            reason: None,
        }
    }
}

impl<'r> From<Pair<'r, Rule>> for ActionVacation {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let mut action_vacation = Self::default();

        for p in pair.into_inner().into_iter() {
            let inner = match p.clone().into_inner().next() {
                Some(inner) => inner,
                None => p.clone(),
            };

            match p.as_rule() {
                Rule::action_vacation_argument_days => {
                    action_vacation.days = Some(Literal::from(inner).inner());
                }
                Rule::action_vacation_argument_subject => {
                    action_vacation.subject = Some(Literal::from(inner).inner());
                }
                Rule::action_vacation_argument_from => {
                    action_vacation.from = Some(Literal::from(inner).inner());
                }
                Rule::action_vacation_argument_addresses => {
                    action_vacation.addresses = Some(Literal::from(inner).inner());
                }
                Rule::action_vacation_argument_mime => {
                    action_vacation.mime = Some(LiteralTypes::Boolean(true));
                }
                Rule::action_vacation_argument_handle => {
                    action_vacation.handle = Some(Literal::from(inner).inner());
                }
                Rule::action_vacation_reason => {
                    action_vacation.reason = Some(Literal::from(inner).inner());
                }
                _ => {
                    println!("Unknown rule: {:?}", p.as_rule());
                }
            };
        }

        action_vacation
    }
}
