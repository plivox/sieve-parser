// use pest::error::ErrorVariant;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;

const SIEVE_MARKER: u8 = 0xff;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SieveParser;

impl SieveParser {
    pub fn serialize(pairs: &Pairs<'_, Rule>) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        let mut buf = Vec::with_capacity(bincode::serialized_size(pairs)? as usize + 2);

        buf.push(SIEVE_MARKER);
        bincode::serialize_into(&mut buf, pairs).unwrap();
        Ok(buf)
    }
}

// use pest::error::*;

#[derive(Debug)]
pub struct ParserError {
    code: usize,
    // message: String,
    // location: (usize, usize),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

pub fn parse<'r>(src: &'r str) -> Result<Pairs<'r, Rule>, ParserError> {
    match SieveParser::parse(Rule::sieve, &src[..]) {
        Ok(pairs) => Ok(pairs),
        Err(e) => {
            eprintln!("Error: {}", e);

            // let location = match e.location {
            //     InputLocation::Pos(a) => (a, a),
            //     InputLocation::Span((a, b)) => (a, b),
            // };

            // let message = match &e.variant {
            //     ErrorVariant::ParsingError {
            //         positives,
            //         negatives: _s,
            //     } => {
            //         // println!("Error: {:?}", e);
            //         // println!("Error: {:?}", positives);
            //         // println!("Error: {:?}", negatives);
            //         "Parsing error".to_string()
            //     }
            //     ErrorVariant::CustomError { message } => message.to_string(),
            // };

            Err(ParserError {
                code: 404,
                // location,
                // message,
            })
        }
    }
}
