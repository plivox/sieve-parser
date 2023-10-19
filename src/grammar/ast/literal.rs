use pest::iterators::Pair;
use serde::ser::{Serialize, Serializer};
use serde::Serialize as SerializeDerive;

// use crate::grammar::ast::string::String;
use crate::grammar::parser::Rule;

#[derive(Debug, Clone, Eq, PartialEq, SerializeDerive)]
#[serde(untagged)]
pub enum LiteralTypes {
    Array(Vec<Literal>),
    String(String),
    Boolean(bool),
    Number(usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Literal {
    pub inner: LiteralTypes,
}

impl Default for Literal {
    fn default() -> Self {
        Self {
            inner: LiteralTypes::String(String::default()),
        }
    }
}

impl Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'r> From<Pair<'r, Rule>> for Literal {
    fn from(pair: Pair<'r, Rule>) -> Self {
        let inner = match pair.as_rule() {
            Rule::r#true | Rule::r#false => {
                let span = pair.as_span().as_str();

                match span.parse::<bool>() {
                    Ok(v) => LiteralTypes::Boolean(v),
                    Err(_) => {
                        unreachable!("Could not parse string to boolean")
                    }
                }
            }
            Rule::number => {
                let span = pair.as_span().as_str();

                match span.parse::<u32>() {
                    Ok(n) => LiteralTypes::Number(n as usize),
                    Err(e) => {
                        unreachable!("Could not parse string to number: {:?}", e)
                    }
                }
            }
            Rule::string | Rule::multi_line_string => {
                let inner = pair.into_inner().next().unwrap().as_span().as_str();
                LiteralTypes::String(String::from(inner))
            }
            Rule::array => {
                let mut result = Vec::new();

                for p in pair.clone().into_inner() {
                    result.push(Literal::from(p));
                }

                LiteralTypes::Array(result)
            }
            _ => {
                unreachable!("Unexpected {:?} literal type", pair.as_rule())
            }
        };

        Self { inner }
    }
}

impl Literal {
    pub fn inner(&self) -> LiteralTypes {
        self.inner.clone()
    }
}

// impl Serialize for Literal {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match &self.inner {
//             LiteralTypes::Array(a) => {
//                 let mut seq = serializer.serialize_seq(Some(a.len()))?;
//                 for e in a {
//                     seq.serialize_element(e)?;
//                 }
//                 seq.end()
//             }
//             LiteralTypes::String(s) => serializer.serialize_str(s),
//             LiteralTypes::Boolean(_) => serializer.serialize_str("hello"),
//             LiteralTypes::Number(_) => serializer.serialize_str("hello"),
//         }

//         let mut map = serializer.serialize_map(Some(1))?;
//         map.serialize_entry("type", "string")?;
//         map.serialize_entry("value", &self.inner)?;
//         map.end()

//         serializer.serialize_i32(40)
//         seq.serialize_element(e)
//         serializer.serialize_str("hello")
//     }
// }

// impl Serialize for LiteralTypes {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match &self {
//             LiteralTypes::Array(a) => {
//                 let mut seq = serializer.serialize_seq(Some(a.len()))?;
//                 for e in a {
//                     seq.serialize_element(e)?;
//                 }
//                 seq.end()
//             }
//             LiteralTypes::String(s) => serializer.serialize_str(s),
//             LiteralTypes::Boolean(_) => serializer.serialize_str("hello"),
//             LiteralTypes::Number(_) => serializer.serialize_str("hello"),
//         }

//         // serializer.serialize_i32(40)
//         // let mut map = serializer.serialize_map(Some(1))?;
//         // map.serialize_entry("type", "string")?;
//         // map.serialize_entry("value", &self)?;
//         // map.end()
//     }
// }
