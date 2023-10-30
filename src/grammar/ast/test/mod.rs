pub mod address;
pub mod allof;
pub mod anyof;
pub mod boolean;
pub mod envelope;
pub mod exists;
pub mod flag;
pub mod header;
pub mod not;
pub mod size;

use serde::Serialize;

pub use {
    address::TestAddress,
    allof::TestAllof,
    anyof::TestAnyof,
    boolean::{TestFalse, TestTrue},
    envelope::TestEnvelope,
    exists::TestExists,
    flag::TestFlag,
    header::TestHeader,
    not::TestNot,
    size::TestSize,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Test {
    TestAddress(TestAddress),
    TestAllof(TestAllof),
    TestAnyof(TestAnyof),
    TestFalse(TestFalse),
    TestTrue(TestTrue),
    TestEnvelope(TestEnvelope),
    TestExists(TestExists),
    TestFlag(flag::TestFlag),
    TestHeader(TestHeader),
    TestNot(TestNot),
    TestSize(TestSize),
}
