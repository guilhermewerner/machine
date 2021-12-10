#![allow(non_snake_case)]

use ::Machine::*;
use std::fs;

fn main() {
    let mut vm = Machine::New([0, 0, 0, 0]);
    let code = fs::read("Examples/Bytecode.bin").unwrap();
    vm.LoadProgram(&code);
    vm.Execute();
}
