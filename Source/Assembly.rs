use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assembly {
    /// Assembly name.
    pub name: Cow<'static, str>,

    /// Assembly version.
    pub version: u8,

    /// Assembly bytecode source.
    pub source: AssemblySource,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssemblySource {
    /// Source instructions.
    pub text: Vec<u8>,

    /// Statics and embed data.
    pub data: Vec<u8>,
}
