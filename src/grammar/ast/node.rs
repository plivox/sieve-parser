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
    ActionNotify(ActionNotify),
    ActionAddFlag(ActionAddFlag),
    ActionRemoveFlag(ActionRemoveFlag),
    ActionSetFlag(ActionSetFlag),

    TestAddress(TestAddress),
    TestAllof(TestAllof),
    TestAnyof(TestAnyof),
    TestFalse(TestFalse),
    TestTrue(TestTrue),
    TestEnvelope(TestEnvelope),
    TestExists(TestExists),
    TestFlag(TestFlag),
    TestHeader(TestHeader),
    TestNot(TestNot),
    TestSize(TestSize),

    Comment(Comment),
}

pub fn tree<'n>(pairs: Pairs<Rule>, mut nodes: Vec<Box<Node>>) -> Vec<Box<Node>> {
    let mut sieve = pairs.into_iter();

    while let Some(pair) = sieve.next() {
        // println!("while(pair): {:#?}", pair);

        match pair.as_rule() {
            Rule::EOI => {}
            // Special rule for grouping test nodes
            Rule::test => {
                let children = tree(pair.clone().into_inner(), nodes.clone());
                nodes = children;
            }
            _ => {
                let node = match pair.as_rule() {
                    // Controls
                    Rule::ctl_condition_if
                    | Rule::ctl_condition_elsif
                    | Rule::ctl_condition_else => {
                        Node::ControlCondition(ControlCondition::from(pair))
                    }
                    Rule::ctl_require => Node::ControlRequire(ControlRequire::from(pair)),
                    Rule::ctl_stop => Node::ControlStop(ControlStop::from(pair)),

                    // Actions
                    Rule::act_fileinto => Node::ActionFileinto(ActionFileinto::from(pair)),
                    Rule::act_redirect => Node::ActionRedirect(ActionRedirect::from(pair)),
                    Rule::act_keep => Node::ActionKeep(ActionKeep::from(pair)),
                    Rule::act_discard => Node::ActionDiscard(ActionDiscard::from(pair)),
                    Rule::act_vacation => Node::ActionVacation(ActionVacation::from(pair)),
                    Rule::act_reject => Node::ActionReject(ActionReject::from(pair)),
                    Rule::act_notify => Node::ActionNotify(ActionNotify::from(pair)),
                    Rule::act_flag_set => Node::ActionSetFlag(ActionSetFlag::from(pair)),
                    Rule::act_flag_add => Node::ActionAddFlag(ActionAddFlag::from(pair)),
                    Rule::act_flag_remove => Node::ActionRemoveFlag(ActionRemoveFlag::from(pair)),

                    // Tests
                    Rule::tst_address => Node::TestAddress(TestAddress::from(pair)),
                    Rule::tst_allof => Node::TestAllof(TestAllof::from(pair)),
                    Rule::tst_anyof => Node::TestAnyof(TestAnyof::from(pair)),
                    Rule::tst_envelope => Node::TestEnvelope(TestEnvelope::from(pair)),
                    Rule::tst_exists => Node::TestExists(TestExists::from(pair)),
                    Rule::tst_false => Node::TestFalse(TestFalse::from(pair)),
                    Rule::tst_header => Node::TestHeader(TestHeader::from(pair)),
                    Rule::tst_size => Node::TestSize(TestSize::from(pair)),
                    Rule::tst_true => Node::TestTrue(TestTrue::from(pair)),
                    Rule::tst_not => Node::TestNot(TestNot::from(pair)),
                    Rule::tst_has_flag => Node::TestFlag(TestFlag::from(pair)),

                    // Comment
                    Rule::COMMENT | Rule::hash_comment | Rule::bracketed_comment => {
                        Node::Comment(Comment::from(pair))
                    }
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
