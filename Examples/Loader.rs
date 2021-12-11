#![allow(non_snake_case)]

use ::Machine::*;
use bincode::Options;
use std::fs;

fn main() {
    let encoded: Vec<u8> = fs::read("Examples/Assembly.bin").unwrap();

    let options = bincode::options()
        .with_big_endian()
        .with_fixint_encoding()
        .allow_trailing_bytes();

    let assembly: Assembly = options.deserialize(&encoded[..]).unwrap();

    let mut vm = Machine::New([0, 0, 0, 0]);

    vm.LoadProgram(&assembly.source.text);
    vm.Execute();
}
