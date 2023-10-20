use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Quantifiers {
    K,
    M,
    G,
}

impl<'r> From<Pair<'r, Rule>> for Quantifiers {
    fn from(pair: Pair<'r, Rule>) -> Self {
        match pair.as_span().as_str() {
            "K" => Quantifiers::K,
            "M" => Quantifiers::M,
            "G" => Quantifiers::G,
            _ => {
                unreachable!("Unexpected {:?} quantifier", pair.as_rule())
            }
        }
    }
}
