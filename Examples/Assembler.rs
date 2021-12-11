#![allow(non_snake_case)]

use ::Machine::*;
use bincode::Options;
use std::fs;

fn main() {
    let assembly = Assembly {
        name: "Serialization Example".into(),
        version: 1,
        source: AssemblySource {
            text: include_bytes!("Bytecode.bin").to_vec(),
            data: Vec::new(),
        },
    };

    let options = bincode::options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes();

    let encoded: Vec<u8> = options.serialize(&assembly).unwrap();
    fs::write("Examples/Assembly.bin", &encoded).unwrap();

    //let decoded: Assembly = bincode::deserialize(&encoded[..]).unwrap();
    //assert_eq!(assembly, decoded);
}
