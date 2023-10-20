use pest::iterators::Pairs;
use serde::Serialize;

use crate::grammar::parser::Rule;

use super::action::*;
use super::comment::*;
use super::control::*;
use super::test::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Node {
    ControlRequire(ControlRequire),
    ControlCondition(ControlCondition),
    ControlStop(ControlStop),

    ActionFileinto(ActionFileinto),
    ActionRedirect(ActionRedirect),
    ActionKeep(ActionKeep),
    ActionDiscard(ActionDiscard),
    ActionVacation(ActionVacation),
    ActionReject(ActionReject),

    TestAddress(TestAddress),
    TestAllof(TestAllof),
    TestAnyof(TestAnyof),
    TestEnvelope(TestEnvelope),
    TestExists(TestExists),
    TestFalse(TestFalse),
    TestHeader(TestHeader),
    TestSize(TestSize),
    TestTrue(TestTrue),

    Comment(Comment),
}

pub fn tree<'n>(pairs: Pairs<Rule>, mut nodes: Vec<Box<Node>>) -> Vec<Box<Node>> {
    let mut sieve = pairs.into_iter();

    while let Some(pair) = sieve.next() {
        // println!("while(pair): {:?}", pair.as_rule());

        match pair.as_rule() {
            Rule::EOI => {}
            //
            // Special rule for grouping test nodes
            //
            Rule::test => {
                let children = tree(pair.clone().into_inner(), nodes.clone());
                nodes = children;
            }
            _ => {
                let node = match pair.as_rule() {
                    // Controls
                    Rule::control_condition => Node::ControlCondition(ControlCondition::from(pair)),
                    Rule::control_require => Node::ControlRequire(ControlRequire::from(pair)),
                    Rule::control_stop => Node::ControlStop(ControlStop::from(pair)),
                    // Actions
                    Rule::action_fileinto => Node::ActionFileinto(ActionFileinto::from(pair)),
                    Rule::action_redirect => Node::ActionRedirect(ActionRedirect::from(pair)),
                    Rule::action_keep => Node::ActionKeep(ActionKeep::from(pair)),
                    Rule::action_discard => Node::ActionDiscard(ActionDiscard::from(pair)),
                    Rule::action_vacation => Node::ActionVacation(ActionVacation::from(pair)),
                    Rule::action_reject => Node::ActionReject(ActionReject::from(pair)),
                    // Tests
                    Rule::test_address => Node::TestAddress(TestAddress::from(pair)),
                    Rule::test_allof => Node::TestAllof(TestAllof::from(pair)),
                    Rule::test_anyof => Node::TestAnyof(TestAnyof::from(pair)),
                    Rule::test_envelope => Node::TestEnvelope(TestEnvelope::from(pair)),
                    Rule::test_exists => Node::TestExists(TestExists::from(pair)),
                    Rule::test_false => Node::TestFalse(TestFalse::from(pair)),
                    Rule::test_header => Node::TestHeader(TestHeader::from(pair)),
                    Rule::test_size => Node::TestSize(TestSize::from(pair)),
                    Rule::test_true => Node::TestTrue(TestTrue::from(pair)),
                    // Comment
                    Rule::COMMENT => Node::Comment(Comment::from(pair)),
                    _ => {
                        unreachable!("Encountered unexpected rule {:?}", pair.as_rule())
                    }
                };

                nodes.push(Box::new(node));
            }
        }
    }

    nodes
}
