use pest::iterators::Pair;
use serde::Serialize;

use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Comparators {
    Is,
    Matches,
    Contains,
    Count,
    Regex,
}

impl<'r> From<Pair<'r, Rule>> for Comparators {
    fn from(pair: Pair<'r, Rule>) -> Self {
        match pair.as_span().as_str() {
            ":is" => Comparators::Is,
            ":matches" => Comparators::Matches,
            ":contains" => Comparators::Contains,
            ":count" => Comparators::Count,
            ":regex" => Comparators::Regex,
            _ => {
                unreachable!("Unexpected {:?} comparator", pair.as_rule())
            }
        }
    }
}
