#![allow(non_snake_case)]

use ::Machine::*;
use std::fs;

fn main() {
    let mut vm = Machine::New(0x00);

    let bytecode =  fs::read("Examples/Simple.bin").unwrap();

    vm.LoadProgram(&bytecode, 0x00);

    vm.Execute();
}
