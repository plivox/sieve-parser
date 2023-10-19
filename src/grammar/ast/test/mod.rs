pub mod address;
pub mod allof;
pub mod anyof;
pub mod boolean;
pub mod envelope;
pub mod exists;
pub mod header;
pub mod size;

use serde::Serialize;

pub use {
    address::TestAddress,
    allof::TestAllof,
    anyof::TestAnyof,
    boolean::{TestFalse, TestTrue},
    envelope::TestEnvelope,
    exists::TestExists,
    header::TestHeader,
    size::TestSize,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Test {
    TestAddress(TestAddress),
    TestAllof(TestAllof),
    TestAnyof(TestAnyof),
    TestEnvelope(TestEnvelope),
    TestExists(TestExists),
    TestFalse(TestFalse),
    TestHeader(TestHeader),
    TestSize(TestSize),
    TestTrue(TestTrue),
}
